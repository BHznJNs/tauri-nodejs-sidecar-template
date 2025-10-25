## 技术栈

前端和后端的 Node.js Sidecar 统一使用 TypeScript 编写

### 前端部分

前端框架：React 19
样式：TailwindCSS
组件库：Shadcn-ui + Vercel AI Elements

### Node.js Sidecar

后端框架：fastify
数据库及 ORM：sqlite + drizzle
LLM API 请求：Vercel AI SDK

## 开发指南

### 上下文信息要求

- 在编码前至少分析 3 个现有实现或模式，识别可复用的接口与约束。
- 绘制依赖与集成点，确认输入输出协议、配置与环境需求。
- 弄清现有测试框架、命名约定和格式化规则，确保输出与代码库保持一致。

### 代码风格

#### 前后端 TypeScript 代码风格

- 字符串统一使用双引号

#### 前端代码风格

- **样式规范**: 使用 TailwindCSS 类名，禁止写 css 文件
- **页面路由**: 在 `frontend/src/pages/` 新增组件即自动成为路由
- **组件编写**: 统一使用 `function Component(props: Props) {}` 格式编写
- **错误处理**: 在出现错误时统一使用 toast 组件展示

#### UI 风格细节

- **按钮英文文本大小写**：使用“句子大小写”，如 "Add to cart", "Create new account"
- **设置界面设置项标题英文文本的大小写**：使用“句子大小写”
- **对话框 (dialog) 组件标题英文文本的大小写**：使用“句子大小写”
- **Label 组件英文文本的大小写**：使用“句子大小写”
