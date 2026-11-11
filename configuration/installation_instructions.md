To connect Wiro AI, add the API key of a Wiro AI project:

1. Sign in at [wiro.ai](https://wiro.ai) and open your [projects](https://wiro.ai/panel/project). If you have none yet, [create a project](https://wiro.ai/panel/project/new).
2. Copy the project's API key into `wiro_api_key`.
3. If the project uses signature-based authentication, copy its API secret into `wiro_api_secret` as well. For an API Key Only project, leave it empty.

Zed installs the server (`@wiro-ai/wiro-mcp` from npm) and runs it with its own Node.js, so there is nothing else to install.

Searching models and reading their prices is free. Running a model or a workflow, and messages to Studio or your agents, are charged to the project's wallet.
