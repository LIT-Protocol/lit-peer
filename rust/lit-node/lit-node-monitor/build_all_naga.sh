echo "Building naga-test"
trunk build --release --features="naga-test" --public-url "https://lit-protocol.github.io/monitor/naga-test/" --dist="naga-test"

echo "Building naga-prod"
trunk build --release --features="naga-prod" --public-url "https://lit-protocol.github.io/monitor/naga-prod/" --dist="naga-prod"

echo "Building naga-dev"
trunk build --release --features="naga-dev" --public-url "https://lit-protocol.github.io/monitor/naga-dev/" --dist="naga-dev"

echo "Building naga-proto"
trunk build --release --features="naga-proto" --public-url "https://lit-protocol.github.io/monitor/naga-proto/" --dist="naga-proto"

echo "Building naga-staging"
trunk build --release --features="naga-staging" --public-url "https://lit-protocol.github.io/monitor/naga-staging/" --dist="naga-staging"

echo "Building internalDev"
trunk build --release --features="internalDev" --public-url "https://lit-protocol.github.io/monitor/internalDev/" --dist="internalDev"

echo "Done"