# Enterprise Profiles

ADA allows IT Administrators to hardcode security constraints globally via the `EnterpriseProfile` configurations. 
By placing a `profile.yaml` inside the ADA install root, enterprises can completely isolate ADA behaviors entirely independently of user requests or prompts.

## 1. `allowed_domains` Array
Whitelists the exact FQDN root domains allowed when the `os.http` or `net.curl` tool invocations trigger. Egress connection attempts outside this domain block are hard-killed at the ADA Rust `Target Guard`.

## 2. `restricted_paths` Array
Defines the `chroot`-like boundary where local execution operations (read, write, delete, traverse) can operate. Highly suggested limiting variables to explicitly `$HOME/.ada` workspaces.

## 3. `cloud_enabled` Flag
When explicitly shifted to `false`, ADA operates in **Air-Gapped Local Mode**. LLM remote API generation triggers a `CloudDisabledError`. Organizations configure local `IntentTaxonomies` mapping strictly deterministic templates, and manually bypass generation via the CLI:
`cargo run -p ada-cli -- run --plan custom_playbook.yaml`
