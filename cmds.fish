#
# This contains fish shell commands to:
# - build and deploy the component
# - interact with the component once it's deployed

alias gli golem-cli
alias build="cargo component build"

################################
# building the KV store
function add_component
  golem-cli component add --component-name slkvs target/wasm32-wasi/release/slkvs.wasm
end

# first-time only
function deploy --description "for the first time deploy of a component with its worker"
  cargo component build --release || return 1
  golem-cli worker delete --worker-name fst --component-name slkvs
  golem-cli component update --component-name slkvs target/wasm32-wasi/release/slkvs.wasm
  golem-cli worker add --worker-name fst --component-name slkvs
end

# for updates use this
function redeploy -a version --description "redeploy to a version"
  cargo component build --release || return 1

  # Use `golem-cli component update` to figure out which version to use.
  # Normal output is something like
  # Updated component with ID 3825415a-f2f9-42dc-99d3-715ff89690a0. New version: 1. Component size is 168531 bytes.
  set result_msg (golem-cli component update --component-name slkvs target/wasm32-wasi/release/slkvs.wasm)

  # extract version
  set captures (string match --regex -g 'Updated component with ID (.*?) New version: (\d+). Component size is (\d+) bytes.*' $result_msg)
  if test $status -ne 0
    echo Failed to match output from `golem-cli component update`: "\n$result_msg"
    return 1
  else
    echo -e (string join \\n $result_msg)
  end
  set target_version $captures[2]

  # do the update
  echo -n "Updating to component version $target_version... "
  golem-cli worker update --worker-name fst --target-version $target_version --mode auto --component-name slkvs
end

function worker_restart
  golem-cli worker delete --worker-name fst --component-name slkvs
  golem-cli worker add --worker-name fst --component-name slkvs
end

################################
# talking to the KV store
function gli_quote
  for v in $argv
    echo -n "$v" | jq -Rs
  end
end

function gli_parameters
  # create an array in fish, since all variables are arrays.
  set -l quoted (gli_quote $argv)
  set joined (string join , $quoted)
  echo "[$joined]"
end

function gli_noquote_parameters
  set joined (string join , $argv)
  echo "[$joined]"
end

function get
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/get \
    --parameters=(gli_parameters $argv[1])
end

function hgettree --description "For a given path, retrieve the entire subtree, with output in json"
  set component_id (gli_component_id)
  set worker_name fst
  set function_name golem:component/api/gettree

  set params "{\"params\": $(gli_parameters $argv[1])}"
  set url "http://localhost:9881/v2/components/$component_id/workers/$worker_name/invoke-and-await?function=$function_name&calling-convention=Component"
  echo -e (curl --silent --json $params $url) | jq .result[0] | string unescape | jq .
end

function gettree --description "For a given path, retrieve the entire subtree, with output in WAVE"
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/gettree \
    --parameters=(gli_parameters $argv[1])
end

function delete  --description "For a given path, delete the value. Fails on a subtree."
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/delete \
    --parameters=(gli_parameters $argv[1])
end

function add --description "For a given path, add the value."
  golem-cli worker invoke-and-await --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/add \
    --parameters=(gli_parameters $argv[1] $argv[2])
end

function listpaths
  golem-cli worker invoke-and-await \
    --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/listpaths \
    --parameters=(gli_parameters $argv[1])
end

function gli_component_id
  set result_msg (gli component get --component-name slkvs)
  set captures (string match --regex -g 'Component with ID (.*?). Version: (\d+). Component size is (\d+) bytes.*' $result_msg)
  if test $status -ne 0
    echo Failed to match output from `golem-cli component update`: "\n$result_msg"
    return 1
  end

  set component_id $captures[1]
  echo $component_id
end

function hlistpaths --description "List all paths, with output in json rather than WAVE"
  set component_id (gli_component_id)
  set worker_name fst
  set function_name golem:component/api/listpaths
  set json_rsp (curl --silent --json '{"params": []}' "http://localhost:9881/v2/components/$component_id/workers/$worker_name/invoke-and-await?function=$function_name&calling-convention=Component")
  # because golem api returns this in a top-level result: []
  echo $json_rsp | jq .result[0]
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
    --parameters=(gli_noquote_parameters (gli_quote $argv[1]) $escaped_tree)
end

function drop --description "Remove all key->values"
  golem-cli worker invoke-and-await \
    --component-name=slkvs \
    --worker-name=fst \
    --function=golem:component/api/drop
end
