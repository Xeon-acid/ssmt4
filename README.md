# 仍在开发测试中

请暂时先使用SSMT3 + TheHerta3系列工具，SSMT4仍在开发测试中。

# SSMT4

第四代超简单3Dmigoto工具箱(Super Simple Migoto Tools 4th)



使用文档与开发笔记:


# AI辅助编程

本工具全程使用Github Copilot的Gemini3 Pro(Preview)进行辅助开发，因为我是几乎一点不会Rust + Tauri以及前端这些技术。

# 为什么更换到Rust + Tauri架构？

- 基于前端渲染技术与Blender进行API交互的更强的完全可控蓝图系统的Mod生成蓝图功能
- 由Rust + Tauri 带来的更小的体积，更快的速度，更高的安全性，跨平台，等等等等
- 使用前端技术可实现任意复杂视觉效果，突破WinUI3效果限制，更精美的界面
- 开发十分方便，AI能够非常成熟的处理前端内容生成，实时修改并查看预览，解放开发效率
- 更简单方便快捷的打包发布流程，开发者友好
- 相比于SSMT3来说使用Rust + Tauri实现全方面的架构升级，使得在WinUI3上受限制的一些视觉效果能够轻易使用前端技术实现
- 对于Mod制作来说的一些复杂需求，也只有前端技术能够很好的展现
- 对于Mod管理来说，前端技术可以实现非常优质的Mod管理功能
- 为未来做好准备，虽然SSMT3仅基于3Dmigoto但是未来可能会出现更强的DX12、Vulkan、Mental FX等Mod制作技术，这次转用跨平台架构就是为未来做准备的(虽然这可能是遥遥无期，但是鉴于SSMT是一个兴趣维护的项目，开发中的乐趣更加重要)。
- 练习编程技术，尤其是Rust以及前端相关技术，为未来其他工具开发、网站开发等做好准备。

总结来说，这是一次革命性的架构变更，将彻底解放由WinUI3架构限制导致部分需求无法实现的问题，并且大大优化了开发和运维成本，几乎每个方面都有巨大的提升。

Python也能实现上述需求，为什么不用基于Python的架构，就像XXMI系列工具一样？

因为Python能实现的效果过于简陋，Rust + Tauri相当于Pro Max版本的Python，能实现更多效果，即使成本确实很高，但我相信对于充满热爱的开发者来说，学习成本并不是问题，何况还有AI的辅助，在SSMT4的开发中我几乎全程使用Gemini3 Pro进行开发，效率提升了几百倍，边看小说边和AI对线就把工具开发好了。

# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
