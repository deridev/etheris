use crate::prelude::*;

#[command("Leia o tutorial para aprender como usar Etheris!")]
#[name("tutorial")]
pub async fn tutorial(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let first_page = EmbedBuilder::new_common()
        .set_author_to_user(&author)
        .set_description(format!("## {} Etheris - O Básico\nEtheris é um bot de **RPG** e **exploração**. Seu objetivo você mesmo define!\nEm Etheris, você controla um **personagem** que pode ser criado através do comando **/registrar**. O seu personagem terá alguns atributos e uma aparência, e todos podem ser vistos através do comando **/perfil**.\n\nEther é a energia vital e mágica que permeia os seres vivos em Etheris, e permite habilidades em batalhas. Quando criar um personagem, você poderá escolher algumas **personalidades** para o seu personagem: personalidades afetam as habilidades que seu personagem tem facilidade em aprender. O seu personagem pode viajar entre regiões do mundo de Etheris, e cada região tem uma peculiaridade e inimigos diferentes para enfrentar. Existem regiões que são cidades, onde seu personagem pode descansar, **trabalhar** com **/trabalhar** e **comprar itens** em **/loja**!\n\n-> Na próxima página, você verá como começar sua jornada.", emojis::ETHER))
        .add_footer_text("Etheris possui faixa etária de 16+ anos.");

    let second_page = EmbedBuilder::new_common()
        .set_author_to_user(&author)
        .set_description("## ❓ Etheris - Como Começar\nPara começar em Etheris você precisa de um personagem. Crie seu personagem com o comando **/registrar**, escolhendo um nome para ele! *(O nome do seu personagem é importante e será o que vai marcar seu personagem. Escolha ele cuidadosamente pois não pode ser alterado.)*\nCom o personagem criado, ele receberá uma aparência, que você pode ver com **/perfil**. Dependendo das personalidades que você escolher, você pode aprender habilidades diferentes logo ao criar o personagem com o comando **/aprender**. Aprender habilidades custa **conhecimento**, que são pontos que você recebe quando estuda o suficiente.\n\nPara aumentar a força do seu personagem você pode **/treinar** e **/estudar**, ou até **/meditar**. Mas tenha em mente que esses comandos custam **pontos de ação**, que são pontos limitados que recarregam com o tempo. Você pode expandir seus pontos de ação estudando.\n\nPara enfrentar inimigos pela sua região, você pode usar **/caçar** e para aventurar-se pelo mundo de Etheris, você pode usar **/explorar**. Quando juntar muitos orbs (dinheiro), você pode usar **/viajar** para ir para outra região!\nSe quiser enfrentar algum amigo de forma casual ~~ou até a morte~~, você pode usar **/batalhar**!")
    .add_footer_text("Etheris possui faixa etária de 16+ anos.");

    let third_page = EmbedBuilder::new_common()
        .set_author_to_user(&author)
        .set_description(format!("## 💀 Etheris - Detalhes\nEm Etheris o seu personagem tem DUAS barras de vida: a **{} resistência** e a **{} vitalidade**. Quando um inimigo remover sua barra de resistência, você pode escolher entre perder a batalha e ir a nocaute, ou apostar sua vida. Se escolher arriscar sua vida, o dano vai pra barra de vitalidade. Se a barra de vitalidade zerar, o seu personagem **MORRE**.\nSe um personagem morrer, você NUNCA MAIS poderá usar ele. Você perde pra sempre e precisa recomeçar do zero. Por isso, apostar a vida é algo perigoso que deve ser evitado ao máximo. Não vá a uma batalha sem **/descansar** antes, se possível!", emojis::RESISTANCE, emojis::VITALITY))
        .add_footer_text("Etheris possui faixa etária de 16+ anos.");

    let fourth_page = EmbedBuilder::new_common()
        .set_author_to_user(&author)
        .set_description("## 🧘 Etheris - Comandos Úteis\n**/estatísticas** - mostra as estatísticas de vitória, derrota, nocautes e etc de algum personagem.\n**/habilidades** - mostra todas suas habilidades.\n**/equipar** - equipe um item cosmético ou um item como arma no seu personagem. Você pode remover com /desequipar.\n**/ler** - alguns itens podem ser lidos como livros, esse comando é usado parar ler eles.\n**/consumir** - itens que podem ser comidos ou bebidos podem ser usados através desse comando.\n**/usar** - alguns itens podem ser usados no seu personagem\n**/habilidade analisar** - analise os detalhes de alguma habilidade que você conhece ao custo de um ponto de ação.\n**/inventário** - veja seus itens.")
        .add_footer_text("Etheris possui faixa etária de 16+ anos.");

    let pages = vec![first_page, second_page, third_page, fourth_page];

    EmbedPagination::new(ctx, pages)
        .set_ephemeral()
        .send()
        .await?;

    Ok(())
}
