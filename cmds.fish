alias gli=golem-cli
alias build="cargo component build"

function add_component
  golem-cli component add --component-name slkvs target/wasm32-wasi/release/slkvs.wasm
end

function get
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/get \
    --parameters="[\"$argv[1]\"]"
end

function gettree
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/gettree \
    --parameters="[\"$argv[1]\"]"
end

function delete
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/delete \
    --parameters="[\"$argv[1]\"]"
end

function add
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/add \
    --parameters="[\"$argv[1]\", \"$argv[2]\"]"
end

function listpaths
  golem-cli worker invoke-and-await \
    --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/listpaths \
    --parameters=$argv
end

function addtree
  if test -f $argv[2]
    # read json from file
    set json_str (cat $argv[2])
  else
    # read json from command line
    set json_str (echo $argv[2])
  end

  # remove all newlines, and escape json
  # otherwise golem-cli chokes
  set escaped_tree (echo $json_str| tr -d "\n" | jq . -Rs | tr -d "\n")

  golem-cli worker invoke-and-await \
    --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/addtree \
    --parameters="[\"$argv[1]\", $escaped_tree]"
end

function drop
  golem-cli worker invoke-and-await \
    --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/drop
end

function deploy
  cargo component build --release || return 1
  gli worker delete --worker-name fst --component-name slkvs
  gli component update --component-name slkvs target/wasm32-wasi/release/slkvs.wasm
  gli worker add --worker-name fst --component-name slkvs
end

function redeploy -a version --description "redeploy to a version"
  cargo component build --release || return 1
  gli component update --component-name slkvs target/wasm32-wasi/release/slkvs.wasm
  gli worker update --worker-name fst --target-version $argv[1] --mode auto --component-name slkvs
end

function worker_restart
  gli worker delete --worker-name fst --component-name slkvs
  gli worker add --worker-name fst --component-name slkvs
end
