use super::*;
use etheris_discord::Emoji;

pub const ALL_ITEMS: &[Item] = &[
    GOLDEN_ROBOT_POEM,
    OLD_ABANDONED_BASEMENT_DIARY,
    ENTITY_039_REPORT,
    HAKIKO_LEGEND,
];

pub const GOLDEN_ROBOT_POEM: Item = Item {
    identifier: "golden_robot_poem",
    display_name: "Poema do Robô Dourado",
    emoji: Emoji::from_unicode("📜"),
    purchase_properties: PurchaseProperties {
        base_price: 750,
        base_sell_price: 160,
        ..PurchaseProperties::default()
    },
    pages: &[
        Page {
            translate: true,
            title: "The Golden Robot Poem",
            content: r#"An inventor birthed a metal sprite,
            With ropes entwined in knots so tight,
            Its parts, a gleaming, charming sight,
            Yet hidden flaws withheld its light.
            
            Within its frame, a tangled mess,
            Confusion reigned, a stark distress,
            Broken cogs in a wild regress,
            Its golden shine held in duress.
            "#,
        },
        Page {
            translate: true,
            title: "The Golden Robot Poem (Prototype)",
            content: r#"An inventor has created a robot
            With ropes and knots
            Its parts so brilliant and charming
            Bringing shiny gold within
            
            
            Its interior, however
            Was less than charming
            It was confused and broken
            It didn't look so golden
            "#,
        },
    ],
    ..Item::default()
};

pub const OLD_ABANDONED_BASEMENT_DIARY: Item = Item {
    identifier: "old_abandoned_basement_diary",
    display_name: "Diário Antigo de um Porão Abandonado",
    emoji: Emoji::from_unicode("📘"),
    purchase_properties: PurchaseProperties {
        base_price: 1250,
        base_sell_price: 299,
        ..PurchaseProperties::default()
    },
    pages: &[
        Page {
            translate: true,
            title: "15/03/1138",
            content: r#"Today I found a talking pig! He is very cute, and I named him George.
            He tells funny jokes, but he is a little strange sometimes. But no matter! I finally got a friend, I am so happy! 
            George loves to talk about the world and the stars, but he never answers me why he can talk. I am curious!
            I'm afraid my parents will freak out when I show George to them, so I'll hide him in the basement today, and tell them tomorrow.
            "#,
        },
        Page {
            translate: true,
            title: "17/03/1138",
            content: r#"Today I showed George to my parents, and they loved him!
            At first they were scared, but then they thought it was amazing that I had a talking pig as a friend, I am so happy!
            Today George told me something about the planet Etheris, he told me that one day mankind will create amazing things like horseless wagons! He likes to imagine impossible things.
            In the evening George showed me a cave he had found north of my house, there was a shiny crystal there and George told me that this was the crystal that taught him how to speak! He said it was called the Intelligence Crystal.
            I brought the crystal home without George because I was curious about it, and my parents let George sleep in the house tonight!
            "#,
        },
        Page {
            translate: true,
            title: "18/03/1138",
            content: r#"Today I woke up at night and couldn't sleep because of the crystal. I really wanted to know what it does, so I ate it to see if anything happened. I don't think I should have done that.
            A lot of strange and confusing things kept going through my head, but for some reason I felt really good. 
            I went to the cave to see if there were any more crystals, I got lost in there but I found a chest with lots of crystals in it! I quickly ate them all, and quickly found my way out of the cave.
            I was feeling different, it was as if strange things and images were going through my mind. I saw confusing things like iron wagons with wheels, giant flying metallic birds. It was very strange, until now I don't know what's happening to me.
            I guess I shouldn't have eaten the crystal.
            I shouldn't have eaten the crystal.
            I shouldn't.
            I shouldn't.
            I shouldn't.
            "#,
        },
    ],
    ..Item::default()
};

pub const ENTITY_039_REPORT: Item = Item {
    identifier: "entity_039_report",
    display_name: "Relatório da ENTIDADE-039",
    emoji: Emoji::from_unicode("📋"),
    purchase_properties: PurchaseProperties {
        base_price: 3000,
        base_sell_price: 600,
        ..PurchaseProperties::default()
    },
    pages: &[
        Page {
            translate: true,
            title: "Appearance",
            content: r#"Described as a towering 2.37-meter figure, ENTITY-039 embodies a humanoid form swathed in obsidian scales that glisten under any light. 
            Its unnerving visage bears completely white, elongated eyes that seem to penetrate the very soul of any onlooker. 
            Remarkably, the entity lacks both nose and mouth, with a skull proportioned at 2.3 times larger than that of an average human.
        
            Its sinewy arms extend to unnaturally elongated proportions, terminating in a mere triad of elongated fingers adorned with menacing 15-centimeter claws. 
            Interestingly, its lower extremities are reminiscent of equine anatomy, featuring legs that curve with a grace akin to those of a horse. 
            The feet mirror the hands in structure but boast a lengthier build."#,
        },
        Page {
            translate: true,
            title: "Details",
            content: r#"ENTITY-039, an enigmatic being, possesses a seemingly inexplicable ability to manipulate and disintegrate matter upon contact. Within its containment, the entity remains motionless for prolonged periods, fixating its gaze upon a singular point in the room.
        
            While typically docile, ENTITY-039 displays an unprecedented ferocity towards any life form within a 5-meter radius. 
            It accelerates to a staggering speed of 128 km/h, swiftly reducing any approaching entity to its elemental components. 
            Regrettably, ENTITY-039 has been the catalyst for the untimely demise of ~~6~~ 21 researchers to date.
        
            Devoid of vocalization or attempts at escape, the entity exhibits no discernible requirements for sustenance, eschewing both food and water. 
            Curiously, it harbors an inexplicable dread solely towards rabbits. In the presence of these small creatures, ENTITY-039 portrays a distinct sense of fear, invariably retreating at the sight of a rabbit, refraining from aggression and displaying palpable signs of trepidation."#,
        },
    ],
    ..Item::default()
};

pub const HAKIKO_LEGEND: Item = Item {
    identifier: "hakiko_legend",
    display_name: "A Lenda de Hakiko",
    emoji: Emoji::from_unicode("📜"),
    purchase_properties: PurchaseProperties {
        base_price: 5000,
        base_sell_price: 1000,
        ..PurchaseProperties::default()
    },
    pages: &[
        // Page 1: The Birth of a Legend
        Page {
            translate: true,
            title: "The Birth of a Legend",
            content: r#"Hakiko was born into a world engulfed by war. From his earliest days, he witnessed the horrors of conflict firsthand. Yet, even amidst the chaos, Hakiko's spirit remained untainted. He possessed an inherent kindness and a deep yearning for peace.

As he grew older, Hakiko's exceptional talent for manipulating ether became evident. He trained relentlessly, honing his skills and developing innovative techniques. 
His mastery over ether surpassed that of any warrior before him, and his reputation as a prodigy spread throughout the land."#,
        },
        // Page 2: The Rise of the Golden General
        Page {
            translate: true,
            title: "The Rise of the Golden General",
            content: r#"Word of Hakiko's prowess reached the ears of the Emperor, who was desperately seeking a way to end the perpetual war. 
            Recognizing Hakiko's potential, the Emperor appointed him as the General of the Imperial Army.

Hakiko, though reluctant to engage in violence, accepted the position out of a sense of duty and a desire to bring peace to his war-torn world. He quickly rose through the ranks, leading the Imperial Army to a series of stunning victories. 
His innovative tactics and masterful use of ether earned him the moniker "The Golden General."#,
        },
        // Page 3: The Hakikotenchou and the Frozen Hope
        Page {
            translate: true,
            title: "The Hakikotenchou and the Frozen Hope",
            content: r#"The war reached a critical juncture at the Battle of Crimson Valley. Outnumbered and facing imminent defeat, Hakiko knew he had to take drastic measures. He unleashed his ultimate technique, the Hakikotenchou, freezing himself in a state of suspended animation while simultaneously purging the overwhelming ether overload from his body.

Hakiko's sudden disappearance from the battlefield demoralized his troops, but the legend of his eventual return kept their hope alive. The phrase "the war will only end when Hakiko thaws" became a rallying cry, a beacon of hope in the darkest of times."#,
        },
        // Page 4: The Return of the Golden General and the End of War
        Page {
            translate: true,
            title: "The Return of the Golden General and the End of War",
            content: r#"Years turned into decades, and the war continued unabated. Just when all hope seemed lost, a tremor shook the battlefield, and a blinding light erupted from the frozen cocoon where Hakiko lay dormant. The Golden General had returned, his ether reserves replenished and his resolve unwavering.

With renewed vigor, Hakiko led his army in a final, decisive charge. His masterful swordsmanship and overwhelming ether manipulation decimated the enemy ranks. The war, which had raged for generations, finally came to an end."#,
        },
        // Page 5: The Legacy of Hakiko
        Page {
            translate: true,
            title: "The Legacy of Hakiko",
            content: r#"Hakiko's victory ushered in an era of peace and prosperity known as the Golden Age of the Sword. His legend became a cornerstone of Etheris' history, inspiring generations to come. He was not only a warrior of unparalleled skill but also a symbol of hope, perseverance, and the belief that true strength lies not in violence but in wisdom and self-control.

Though the Golden Age of the Sword eventually faded, Hakiko's legacy continues to shape the world of Etheris. His innovative techniques are still studied by aspiring warriors, and his philosophy of peace and self-mastery continues to resonate with those who seek true strength."#,
        },
    ],
    ..Item::default()
};
