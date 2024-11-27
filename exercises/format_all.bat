for /d %%D in (*) do (
cd %%D
cargo fmt
cd ..
)