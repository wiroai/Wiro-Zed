<div align="center">

<img src=".github/assets/wiro-icon.png" alt="Wiro AI" width="96">

# Wiro AI MCP Server for Zed

Use Wiro AI from Zed's Agent Panel: search 1,000+ image, video, audio and LLM models, check their prices and run them, build and run workflows, and talk to Studio and your own agents.

[Website](https://wiro.ai) · [Models](https://wiro.ai/models) · [MCP docs](https://wiro.ai/docs/wiro-mcp-server) · [Dashboard](https://wiro.ai/panel)

</div>

This [Zed](https://zed.dev) extension adds the official [Wiro AI MCP server](https://github.com/wiroai/Wiro-MCP) to Zed. Zed downloads the server, [`@wiro-ai/wiro-mcp`](https://www.npmjs.com/package/@wiro-ai/wiro-mcp), from npm and runs it with the Node.js runtime it manages itself, so there is nothing else to install. Each time the server starts, the extension checks npm for a newer version.

![Zed's Agent Panel running a Wiro AI image model and showing the result](docs/screenshots/agent-panel-model-run.png)

## Install

1. In Zed, open the command palette, run `zed: extensions`, search for **Wiro AI MCP Server** and click **Install**. You can also open **Settings → AI → MCP Servers**, click **Add Server** and choose **Install from Extensions**.
2. Zed opens a configuration window. Put your project's API key in `wiro_api_key`, and its API secret in `wiro_api_secret` if the project uses signature-based authentication. Then click **Configure Server**.
3. In **Settings → AI → MCP Servers**, the dot next to **Wiro AI** turns green when the server is running. If it does not, its tooltip says why, and `zed: open log` shows more detail.

## Get an API key

Sign up at [wiro.ai](https://wiro.ai) and create a project at [wiro.ai/panel/project/new](https://wiro.ai/panel/project/new). Your projects and their keys are listed at [wiro.ai/panel/project](https://wiro.ai/panel/project).

You choose the authentication type when you create the project:

- **Signature-based**: set both `wiro_api_key` and `wiro_api_secret`.
- **API Key Only**: set `wiro_api_key` and leave `wiro_api_secret` empty.

## Settings

The configuration window saves your settings in Zed's `settings.json`. To change them later, run `zed: open settings file` and edit the `mcp-server-wiro` entry:

```json
{
  "context_servers": {
    "mcp-server-wiro": {
      "settings": {
        "wiro_api_key": "your-api-key",
        "wiro_api_secret": "your-api-secret",
        "toolsets": ["models", "workflows"]
      }
    }
  }
}
```

| Setting | Required | Description |
| --- | --- | --- |
| `wiro_api_key` | Yes | API key of your Wiro AI project |
| `wiro_api_secret` | For signature-based projects | API secret of the same project |
| `toolsets` | No | Tool groups to load: any of `models`, `workflows` and `agents`. All groups load when it is left out |

## Tools

With all groups loaded, the server gives the Agent Panel 50 tools:

| Group | Tools |
| --- | --- |
| Always loaded | `get_account`, `get_playbook`, `search_docs` |
| `models` | `search_models`, `get_model_schema`, `recommend_model`, `explore`, `run_model`, `wait_for_task`, `get_task`, `list_tasks`, `get_task_price`, `cancel_task`, `kill_task`, `upload_file` |
| `workflows` | `list_workflows`, `get_workflow`, `create_workflow`, `update_workflow`, `clone_workflow`, `delete_workflow`, `run_workflow`, `wait_for_workflow_run`, `resume_workflow_run`, `list_workflow_runs` |
| `agents` | `list_agent_templates`, `get_agent_template`, `list_agent_skills`, `deploy_agent`, `list_my_agents`, `get_my_agent`, `control_agent`, `update_agent`, `apply_agent_skills`, `set_agent_credentials`, `connect_agent_oauth`, `disconnect_agent_oauth`, `manage_agent_custom_skill`, `manage_agent_billing`, `get_agent_activity`, `delete_agent`, `list_projects`, `bind_agent_project` |
| `workflows` or `agents` | `send_agent_message`, `get_agent_message`, `get_agent_conversation`, `cancel_agent_message`, `delete_agent_session`, `list_studio_results`, `save_workflow_from_agent` |

`workflows` also loads the model tools a workflow needs: `search_models`, `get_model_schema`, `recommend_model` and `upload_file`. What each tool does is described in the [Wiro MCP server README](https://github.com/wiroai/Wiro-MCP#available-tools). The server also offers four MCP prompts: `build_workflow`, `deploy_agent`, `create_with_studio` and `talk_to_my_agent`.

![Wiro AI model tools in the Configure MCP Tools list of Zed's Agent Panel](docs/screenshots/agent-panel-tools.png)

## Try it

Open the Agent Panel and ask, for example:

- *"Find three text-to-image models on Wiro AI and compare their prices"*
- *"Show me the parameters for google/veo3-1"*
- *"Generate a photorealistic image of a mountain lake at sunset"*
- *"Build a workflow that turns a product photo into a short ad video"*
- *"Which agent templates can manage Instagram? What does each plan cost?"*

Searching models and reading their parameters and prices is free. Running a model or a workflow, and messages to Studio or the Workflow Agent, are charged to the project's wallet. Messages to your own agents use the agent's credits. Tools with a known price, such as deploying an agent or buying credits, show the price first and act only after you confirm. Zed also asks you to approve each tool call unless you change its [tool permissions](https://zed.dev/docs/ai/tool-permissions).

## Hosted server

Zed can also connect to the hosted Wiro AI MCP server at `https://mcp.wiro.ai/v1` without this extension: add it as a remote server in **Settings → AI → MCP Servers** and sign in with your Wiro AI account when Zed asks. See the [MCP docs](https://wiro.ai/docs/wiro-mcp-server) for details.

## Development

1. Install Rust with [rustup](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. In Zed, run `zed: install dev extension` and pick the cloned folder.

`cargo test` runs the unit tests.

## More from Wiro AI

| Repository | What it is |
| --- | --- |
| [Wiro-MCP](https://github.com/wiroai/Wiro-MCP) | Official Wiro AI MCP server, [`@wiro-ai/wiro-mcp` on npm](https://www.npmjs.com/package/@wiro-ai/wiro-mcp) |
| [Wiro-Plugin](https://github.com/wiroai/Wiro-Plugin) | Wiro AI plugin for Cursor, Claude Code, Gemini CLI, Devin, Antigravity and Kiro |
| [Wiro-Skills](https://github.com/wiroai/Wiro-Skills) | Agent skills for images, image edits, video, music, speech, transcripts, 3D and model search |
| [Wiro-LLM](https://github.com/wiroai/Wiro-LLM) | Vercel AI SDK provider for chat, images, speech, transcription and video, [`@wiro-ai/wiro-llm` on npm](https://www.npmjs.com/package/@wiro-ai/wiro-llm) |
| [Wiro-Chromium-Extension](https://github.com/wiroai/Wiro-Chromium-Extension) | Wiro AI extension for Chrome, Edge, Opera and Brave |
| [Wiro-Firefox-Extension](https://github.com/wiroai/Wiro-Firefox-Extension) | Wiro AI add-on for Firefox |
| [Wiro-Safari-Extension](https://github.com/wiroai/Wiro-Safari-Extension) | Wiro AI extension for Safari on the Mac |
| [Swift-QuickStart](https://github.com/wiroai/Swift-QuickStart) | WiroKit, the Wiro AI SDK for iOS |
| [Kotlin-QuickStart](https://github.com/wiroai/Kotlin-QuickStart) | WiroKit, the Wiro AI SDK for Android |
| [Dart-QuickStart](https://github.com/wiroai/Dart-QuickStart) | Wiro AI SDK for Dart and Flutter, [`wiro` on pub.dev](https://pub.dev/packages/wiro) |
| [ReactNative-QuickStart](https://github.com/wiroai/ReactNative-QuickStart) | Wiro AI SDK for React Native and Expo, [`@wiro-ai/wiro-react-native` on npm](https://www.npmjs.com/package/@wiro-ai/wiro-react-native) |
| [Swift-Realtime](https://github.com/wiroai/Swift-Realtime) | Realtime voice AI example for iOS |
| [Kotlin-Realtime](https://github.com/wiroai/Kotlin-Realtime) | Realtime voice AI example for Android |
| [Dart-Realtime](https://github.com/wiroai/Dart-Realtime) | Realtime voice AI example for Flutter |
| [ReactNative-Realtime](https://github.com/wiroai/ReactNative-Realtime) | Realtime voice AI example for React Native |
| [n8n-nodes-wiroai](https://github.com/wiroai/n8n-nodes-wiroai) | Wiro AI nodes for n8n, [`@wiro-ai/n8n-nodes-wiroai` on npm](https://www.npmjs.com/package/@wiro-ai/n8n-nodes-wiroai) |
| [n8n-nodes-wiro](https://github.com/wiroai/n8n-nodes-wiro) | One Wiro AI node for n8n with a searchable model picker, [`@wiro-ai/n8n-nodes-wiro` on npm](https://www.npmjs.com/package/@wiro-ai/n8n-nodes-wiro) |
| [Wiro-Dify-Plugin](https://github.com/wiroai/Wiro-Dify-Plugin) | Wiro AI plugins for Dify: the LLM gateway as a model provider, plus image, video, speech and music tools |
| [Wiro-ComfyUI](https://github.com/wiroai/Wiro-ComfyUI) | Wiro AI nodes for ComfyUI |
| [Wiro-Hermes](https://github.com/wiroai/Wiro-Hermes) | Wiro AI plugins for Hermes Agent: image, video, audio and 3D tools, plus the LLM gateway as a model provider |
| [Wiro-Action](https://github.com/wiroai/Wiro-Action) | GitHub Action for Wiro AI models and the LLM gateway, [on the GitHub Marketplace](https://github.com/marketplace/actions/wiro-ai-action) |
| [Wiro-Raycast](https://github.com/wiroai/Wiro-Raycast) | Wiro AI extension for Raycast: image and video generation, LLM chat and model search |
| [Wiro-WordPress](https://github.com/wiroai/Wiro-WordPress) | Wiro AI plugin for WordPress: featured images, image edits, alt text, narration and SEO suggestions |
| [Wiro-Obsidian](https://github.com/wiroai/Wiro-Obsidian) | Wiro AI plugin for Obsidian: images, video, speech, music, transcription and text tools |
| [Wiro-Blender](https://github.com/wiroai/Wiro-Blender) | Wiro AI add-on for Blender: text to 3D, image to 3D and textures |
| [Wiro-Figma](https://github.com/wiroai/Wiro-Figma) | Wiro AI plugin for Figma: images, vectors, image edits and text |
| [Wiro-Next-Starter](https://github.com/wiroai/Wiro-Next-Starter) | Next.js starter for Wiro AI: Playground, image, video and audio tools, and LLM chat, ready for Vercel and Railway |
| [Wiro-Docs](https://github.com/wiroai/Wiro-Docs) | Source of the Wiro AI API docs |

[Website](https://wiro.ai) · [Models](https://wiro.ai/models) · [Docs](https://wiro.ai/docs) · [X](https://x.com/wiroai)

## License

MIT, see [LICENSE](./LICENSE).
