<div align="center">

<img src="https://s2.loli.net/2023/03/26/4URd1BKj3ToycLl.png" width=200 />

# meme-generator-rs

_✨ 表情包生成器，用于制作各种沙雕表情包 ✨_

<p align="center">
  <img src="https://img.shields.io/github/license/MemeCrafters/meme-generator-rs">
  <a href="https://crates.io/crates/meme_generator">
    <img src="https://img.shields.io/crates/v/meme_generator">
  </a>
  <a href="https://pypi.org/project/meme-generator">
    <img src="https://img.shields.io/pypi/v/meme-generator">
  </a>
  <a href="https://jq.qq.com/?_wv=1027&k=wDVNrMdr">
    <img src="https://img.shields.io/badge/QQ%E7%BE%A4-682145034-orange">
  </a>
</p>

</div>

## 表情列表

表情详细信息、表情预览等可以在 [--> 表情列表 <--](https://github.com/MemeCrafters/meme-generator-rs/wiki/%E8%A1%A8%E6%83%85%E5%88%97%E8%A1%A8) 查看

## 使用、配置

详见 Wiki：[--> Wiki <--](https://github.com/MemeCrafters/meme-generator-rs/wiki)

## 其他

meme-generator-rs 支持通过加载动态链接库的方式加载额外的表情，详见 [加载其他表情](https://github.com/MemeCrafters/meme-generator-rs/wiki/%E5%8A%A0%E8%BD%BD%E5%85%B6%E4%BB%96%E8%A1%A8%E6%83%85)

其他表情仓库：
- [MemeCrafters/meme-generator-contrib-rs](https://github.com/MemeCrafters/meme-generator-contrib-rs) meme-generator-rs 额外表情仓库

## 已知问题

- Windows 下程序无报错退出

需要安装 [Visual C++ 运行时](https://aka.ms/vs/17/release/VC_redist.x64.exe)

相关 Issue：https://github.com/kyamagu/skia-python/issues/289

- Linux 下字体异常

设置 locate 为英文：
```
export LANG=en_US.UTF-8
```

相关 Issue：https://github.com/rust-skia/rust-skia/issues/963

## 声明

本仓库的表情素材等均来自网络，如有侵权请联系作者删除

## 鸣谢

- 感谢本项目以及 [nonebot-plugin-petpet](https://github.com/MemeCrafters/nonebot-plugin-petpet)、[nonebot-plugin-memes](https://github.com/MemeCrafters/nonebot-plugin-memes)、[meme-generator](https://github.com/MemeCrafters/meme-generator) 项目的贡献者们

部分表情素材或代码参考了以下项目，感谢这些项目的开发者们

- [Ailitonia/omega-miya](https://github.com/Ailitonia/omega-miya) 基于 nonebot2 的多平台机器人
- [FloatTech/ZeroBot-Plugin](https://github.com/FloatTech/ZeroBot-Plugin) 基于 ZeroBot 的 OneBot 插件
- [HibiKier/zhenxun_bot](https://github.com/HibiKier/zhenxun_bot) 基于 Nonebot2 开发，非常可爱的绪山真寻bot
- [SAGIRI-kawaii/sagiri-bot](https://github.com/SAGIRI-kawaii/sagiri-bot) 基于Graia Ariadne和Mirai的QQ机器人 SAGIRI-BOT
- [Dituon/petpet](https://github.com/Dituon/petpet) 根据模板生成图像
- [Ice-Hazymoon/MikuTools](https://github.com/Ice-Hazymoon/MikuTools) 一个轻量的工具集合
- [kexue-z/nonebot-plugin-nokia](https://github.com/kexue-z/nonebot-plugin-nokia) 诺基亚手机图生成
- [RafuiiChan/nonebot_plugin_charpic](https://github.com/RafuiiChan/nonebot_plugin_charpic) 字符画生成插件
- [MerCuJerry/nonebot-plugin-batitle](https://github.com/MerCuJerry/nonebot-plugin-batitle) NoneBot 插件 碧蓝档案式标题生成器
