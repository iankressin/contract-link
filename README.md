# Lien

This command-line interface is designed to assist smart-contract developers in seamlessly integrating their contracts' ABIs and addresses from smart-contract repositories directly into their application folder, streamlining the development process.

## Installation

1. Run: 
```bash
curl -L https://raw.githubusercontent.com/iankressin/lien/main/install.sh | bash
```

2. Add `lien` to your `PATH`:
```bash
# In you .bashrc
export PATH="$PATH:$HOME/.lien/bin"
```

## Usage

Lien supports multiple projects, so first run
```bash
lien config

> Pick a name for your project: paipa 
> Enter the path of your contracts project: /home/kig/Projects/paipa/contracts/ #
> Select the framework used by your contracts: Foundry
> Enter the path where you want to put the abi files: /home/kig/Projects/paipa/frontend/src/lib/abis/
> Enter the path where you want to put the addresses file: /home/kig/Projects/paipa/frontend/src/lib/constants/
> Select the framework used by app: Viem
Config file created at: /home/kig/.lien/config
```

Then, once you are ready generate the files on your app folder run:
```bash
lien generate paipa # In your case, the name given to the project in the previous step
```

## Known issues
- Only supports Foundry as smart-contract framework
- The deploy script needs to be called `Deploy.s.sol`

