# frontend/Dockerfile

# --- Estágio de Build ---
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
# Executa o build de produção do Vite
RUN npm run build

# --- Estágio de Servir ---
FROM nginx:stable-alpine
# Remove a configuração padrão do Nginx
RUN rm /etc/nginx/conf.d/default.conf
# Copia a nossa configuração customizada para SPAs
COPY nginx.conf /etc/nginx/conf.d/
# Copia os arquivos estáticos construídos do estágio anterior
COPY --from=builder /app/dist /usr/share/nginx/html
# Expõe a porta 80
EXPOSE 80
# Comando para iniciar o Nginx
CMD ["nginx", "-g", "daemon off;"]
