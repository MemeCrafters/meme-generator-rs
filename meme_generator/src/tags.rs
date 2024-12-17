use std::collections::HashSet;

macro_rules! meme_tags {
    ($($name:ident = ( $base:expr $(, $inherit:ident)* ),)+ $(,)?) => {
        pub(crate) struct MemeTags;

        impl MemeTags {
            $(
                pub fn $name() -> HashSet<String> {
                    #[allow(unused_mut)]
                    let mut set = $base.iter().map(|&tag| tag.to_string()).collect::<HashSet<_>>();
                    $(
                        set.extend(Self::$inherit());
                    )*
                    set
                }
            )+
        }
    };
}

meme_tags!(
    // 米家游戏
    mihoyo = (["米哈游"]),
    // genshin = (["原神"], mihoyo),
    star_rail = (["崩坏：星穹铁道"], mihoyo),
    // honkai3 = (["崩坏3"], mihoyo),
    // nahida = (["纳西妲", "草神"], genshin),
    // hutao = (["胡桃"], genshin),
    // klee = (["可莉"], genshin),
    // keqing = (["刻晴"], genshin),
    // zhongli = (["钟离"], genshin),
    // nilou = (["妮露"], genshin),
    // yae_miko = (["八重神子"], genshin),
    // ayaka = (["神里绫华"], genshin),
    // bronya = (["布洛妮娅·扎伊切克"], honkai3),
    // captain = (["休伯利安号", "舰长"], honkai3),
    // griseo = (["格蕾修"], honkai3),
    firefly = (["流萤"], star_rail),
    // 蔚蓝档案
    blue_archive = (["蔚蓝档案", "碧蓝档案"]),
    // shiroko = (["砂狼白子"], blue_archive),
    // kokona = (["春原心奈", "春原心菜"], blue_archive),
    // plana = (["普拉娜"], blue_archive),
    arona = (["阿罗娜"], blue_archive),
    // 其他
    // capoo = (["猫猫虫", "咖波"]),
    // nekoha = (["猫羽雫"]),
    // kirby = (["星之卡比"]),
    atri = (["亚托莉", "ATRI", "萝卜子"]),
);
