# Terminal 1 - start postgresql

docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# Terminal 2 - build frontend

cd frontend
npm run build

# Terminal 3 - build backend

cd backend
cargo run -- ../frontend/web-folder
