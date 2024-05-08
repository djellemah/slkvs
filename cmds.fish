alias gli=golem-cli
alias build="cargo component build"

function get
  golem-cli worker invoke-and-await \
    --component-name=yoyo \
    --worker-name=fst \
    --function=golem:component/api/get \
    --parameters="[\"$argv[1]\"]"
end

function add
  golem-cli worker invoke-and-await \
    --component-name=yoyo \
    --worker-name=fst \
    --function=golem:component/api/add \
    --parameters="[\"$argv[1]\", \"$argv[2]\"]"
end

function listpaths
  golem-cli worker invoke-and-await \
    --component-name=yoyo \
    --worker-name=fst \
    --function=golem:component/api/listpaths \
    --parameters=$argv
end

function deploy
  cargo component build --release || return 1
  gli worker delete --worker-name fst --component-name yoyo
  gli component update --component-name yoyo target/wasm32-wasi/release/yoyo.wasm
  gli worker add --worker-name fst --component-name yoyo
end

function redeploy -a version --description "redeploy to a version"
  cargo component build --release || return 1
  gli component update --component-name yoyo target/wasm32-wasi/release/yoyo.wasm
  gli worker update --worker-name fst --target-version $argv[1] --mode auto --component-name yoyo
  # gli worker update --worker-name fst --mode auto --component-name yoyo
end
