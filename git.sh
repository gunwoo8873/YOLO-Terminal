# !/bin/env bash

USERNAME=$(whoami)
GitConfigPath="C:/Users/${USERNAME}/.gitconfig"
ExitPath="./function/Exit.sh"

function ReadFile() {
  if [[ -f $GitConfigPath ]]; then
    echo "Checking Complete have git config file"

  elif [[ -e $GitConfigPath ]]; then
    echo "Not found git config file"
    read -p "You want create giconfig file [Y/N]?" $SELECTE_INPUT

    if [[ $SELECTE_INPUT == [yY] ]]; then
      echo "Creating git config file, wait please"

    elif [[ $SELECTE_INPUT == [nN] ]] && [[ -r $ExitPath ]]; then
      bash $ExitPath
    fi
  fi
}

ReadFile