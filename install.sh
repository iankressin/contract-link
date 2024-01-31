#!/bin/bash
LIEN_DIR="$HOME/.lien"
BIN_DIR="$LIEN_DIR/bin"

function install {
  echo "Installing Lien CLI ..."

  LATEST=$(curl -qs https://api.github.com/repos/iankressin/lien/releases/latest | grep tag_name | head -n 1 | cut -d '"' -f 4);
  URL="https://github.com/iankressin/lien/releases/download/${LATEST}/lien"

  echo $URL

  if [ ! -d $LIEN_DIR ]; then
    echo "--> Configuring directories ..."
    mkdir $LIEN_DIR
    cd $LIEN_DIR
    if [ ! -d $BIN_DIR ]; then
      mkdir $BIN_DIR 
      cd $BIN_DIR
    fi
  else
    cd $BIN_DIR
  fi

  echo "--> Downloading ..."
  bash -c "ls"
  bash -c "curl --fail -# -L $URL > lien"
  BIN="lien"
	  chmod +x $BIN || fail "chmod +x failed"

  echo "*** DONE ***"
  echo "Now, please add $BIN_DIR to your PATH"
}

install
