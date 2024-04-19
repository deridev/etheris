use std::{fmt::Debug, mem::discriminant};

use etheris_common::{Color, Identifiable};
use etheris_discord::{EmbedAuthor, EmbedBuilder, Emoji, UserExtension};

use self::api_input::ApiInput;

use super::{get_boxed_skill_from_kind, prelude::*};
#[derive(Clone, Default)]
pub struct SkillMirror {
    stored_skill: Option<FighterSkill>,
}

impl Debug for SkillMirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkillMirror")
            .field("stored_skill", &self.stored_skill)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UseInput {
    UseCopiedSkill,
    CopyAnotherSkill
}

impl Identifiable for UseInput {
    fn identifier(&self) -> String {
        match self {
            Self::UseCopiedSkill => "use_copied_skill".to_string(),
            Self::CopyAnotherSkill => "copy_another_skill".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Skill for SkillMirror {
    fn kind(&self) -> SkillKind {
        SkillKind::SkillMirror
    }

    fn data(&self, fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "skill_mirror",
            name: "Espelho de Habilidade",
            description: "Copia uma habilidade de um inimigo e a usa contra ele. Custa muita sobrecarga.",
            explanation: "Habilidade de metaconsciência avançada: requer foco e controle do ether para analisar o ether do inimigo e então replicar a habilidade.",
            complexity: SkillComplexity::UltraHard,
            use_cost: SkillCost { 
                ether: if let Some(skill) = &self.stored_skill { 
                    let skill = get_boxed_skill_from_kind(skill.base_kind.clone());
                    skill.data(fighter).use_cost.ether + 15
                } else { 
                    50 
                } 
            },
        }
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);

        if let Some(skill) = &self.stored_skill {
            let skill = get_boxed_skill_from_kind(skill.base_kind.clone());
            display.sub_header.push_str(&format!("\n**Habilidade copiada**: {}", skill.data(fighter).name));
        }

        display
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_>) -> Probability {
        if api.fighter().overload > 125.0 {
            Probability::NEVER
        } else if self.stored_skill.is_some() {
            let skill = get_boxed_skill_from_kind(self.stored_skill.as_ref().unwrap().base_kind.clone());
            skill.ai_chance_to_pick(api)
        } else {
            Probability::new(70)
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        if self.stored_skill.is_none() {
            copy_fighter_skills(self, &mut api, &fighter, &target).await?;
            return Ok(());
        }

        let mut options = vec![
            ApiInput {
                name: "Usar Habilidade Copiada".to_string(),
                description: "Use a habilidade que você já copiou.".to_string(),
                emoji: Some(Emoji::from_unicode("🌀")), 
                active: true,
                ai_weight: 100,
                value: UseInput::UseCopiedSkill,
            },
            ApiInput {
                name: "Copiar Outra Habilidade".to_string(),
                description: "Selecione outra habilidade para copiar.".to_string(),
                emoji: Some(Emoji::from_unicode("📜")), 
                active: true,
                ai_weight: 30,
                value: UseInput::CopyAnotherSkill,
            },
        ];

        if fighter.overload >= 150.0 {
            options[1].active = false;
            options[1].description.push_str("\n-> Sua sobrecarga está muito alta para copiar outra habilidade.");
        }

        let Some(selected_option) = api_input::select_input(&mut api, None, options).await? else {
            return Ok(());
        };

        match selected_option.value {
            UseInput::UseCopiedSkill => {
                if let Some(skill) = &self.stored_skill {
                    skill.dynamic_skill.lock().await.on_use(api).await?;
                }
            }
            UseInput::CopyAnotherSkill => {
                copy_fighter_skills(self, &mut api, &fighter, &target).await?;
            }
        }

        Ok(())
    }

    async fn passive_on_cycle(&mut self, api: BattleApi<'_>) -> SkillResult<()> {
        if let Some(skill) = &mut self.stored_skill {
            skill.dynamic_skill.lock().await.passive_on_cycle(api).await?;
        }

        Ok(())
    }

    async fn passive_fighter_tick(&mut self, api: BattleApi<'_>) -> SkillResult<()> {
        if let Some(skill) = &mut self.stored_skill {
            skill.dynamic_skill.lock().await.passive_fighter_tick(api).await?;
        }

        Ok(())
    }

    async fn passive_on_kill(
        &mut self,
        api: BattleApi<'_>,
        killed: FighterIndex,
    ) -> SkillResult<()> {
        if let Some(skill) = &mut self.stored_skill {
            skill.dynamic_skill.lock().await.passive_on_kill(api, killed).await?;
        }

        Ok(())
    }

    async fn passive_on_damage(
        &mut self,
        api: BattleApi<'_>,
        damage: DamageSpecifier,
    ) -> SkillResult<()> {
        if let Some(skill) = &mut self.stored_skill {
            skill.dynamic_skill.lock().await.passive_on_damage(api, damage).await?;
        }

        Ok(())
    }

    async fn passive_on_damage_miss(
        &mut self,
        api: BattleApi<'_>,
        damage: DamageSpecifier,
    ) -> SkillResult<()> {
        if let Some(skill) = &mut self.stored_skill {
            skill.dynamic_skill.lock().await.passive_on_damage_miss(api, damage).await?;
        }

        Ok(())
    }
}

async fn copy_fighter_skills(
    skill: &mut SkillMirror,
    api: &mut BattleApi<'_>,
    fighter: &Fighter,
    target: &Fighter,
) -> anyhow::Result<()> {
    let mut enemy_skills = vec![];
    for skill in target.skills.iter() {
        let skill_kind = skill.dynamic_skill.lock().await.kind();
        if fighter.skills.iter().any(|s| discriminant(&s.base_kind) == discriminant(&skill_kind)) {
            continue;
        }

        enemy_skills.push(skill_kind);
    }

    if enemy_skills.is_empty() {
        api.emit_message(format!("**{}** não tem nenhuma habilidade que você possa copiar!", target.name));
        return Ok(());
    }

    let mut inputs = vec![];
    let mut embed = EmbedBuilder::new_common()
        .set_color(Color::DARK_YELLOW)
        .set_author(EmbedAuthor {
            name: "Escolha uma habilidade para copiar!".to_string(),
            icon_url: fighter.user.as_ref().map(|u| u.avatar_url()),
        });

    for skill in enemy_skills.iter() {
        let can_copy = (skill.intelligence_requirement() as f32) <= ((fighter.intelligence_level + 5) as f32 * 1.25);
        let skill = get_boxed_skill_from_kind(skill.clone());
        let complexity = skill.data(fighter).complexity;

        inputs.push(ApiInput {
            name: skill.data(target).name.to_string(),
            description: skill.data(target).description.to_string(),
            emoji: None,
            active: can_copy,
            value: skill.kind(),
            ai_weight: match complexity {
                SkillComplexity::VerySimple | SkillComplexity::Simple => 10,
                SkillComplexity::Normal => 25,
                SkillComplexity::Hard => 40,
                SkillComplexity::VeryHard => 60,
                SkillComplexity::UltraHard => 80,
                SkillComplexity::BeginnerMaster => 100,
                SkillComplexity::Master => 125,
                SkillComplexity::SuperMaster => 150,
            }
        });

        embed = embed.add_inlined_field(
            skill.data(target).name.to_string(),
            if !can_copy {
                "**Habilidade complexa demais para você copiar**".to_string()
            } else {
                skill.data(target).description.to_string()
            },
        );
    }

    let Some(selected_skill) = api_input::select_input(api, Some(embed), inputs).await? else {
        return Ok(());
    };

    let copied_skill = FighterSkill::new(get_boxed_skill_from_kind(selected_skill.value)); 

    skill.stored_skill = Some(copied_skill);
    api.add_overload(api.fighter_index, 30.0).await;
    api.emit_message(format!("**{}** copiou a habilidade **{}** de **{}**!", fighter.name, selected_skill.name, target.name));
    Ok(())
}