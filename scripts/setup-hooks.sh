#!/bin/sh

mkdir -p .git/hooks

cat << 'EOF' > .git/hooks/pre-commit
#!/bin/sh
pnpm run format
cd src-tauri && cargo fmt
EOF

chmod +x .git/hooks/pre-commit

echo "Hooks setup successfully"
