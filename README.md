## Hatchr

Short links with x402 paywalls on Solana. Creators set a price in SOL; visitors pay via x402 and are redirected to the original URL. Hatchr takes a 3% platform fee.

Built for the Solana X402 Hackathon. See rules and timeline: [Solana X402 Hackathon](https://solana.com/x402/hackathon)

### Why Hatchr?

- **For creators**: Monetize links instantly without standing up your own paywall or backend.
- **For users**: One-tap payment with wallet (e.g., Phantom), immediate access after payment.
- **For agents**: x402-compatible flows enable agent-to-resource micropayments by design.

---

## MVP Scope

1. **Connect wallet** (Phantom or any Solana wallet via wallet adapter)
2. **Create a paid short link** by pasting a destination URL, setting a price in SOL, and optionally specifying a recipient public key (defaults to the creator’s connected wallet)
3. **View and manage links**: list of created links with basic stats (views, payments, revenue)

### Payment and Fees

- Currency: **SOL** on Solana (devnet for MVP)
- Protocol: **x402** (HTTP 402 Payment Required challenge)
- Platform fee: **3%** of each successful payment to the Hatchr treasury

---

## How It Works

### Creator Flow

1. Connect wallet
2. Create link: provide `targetUrl`, `priceSol`, and optional `recipientPubkey` (defaults to the connected wallet pubkey)
3. Receive a short link like `hatchr.xyz/l/abc123`
4. Share the short link; revenue flows to the recipient wallet less the 3% platform fee

### Visitor Flow (x402)

1. Open a Hatchr short link `hatchr.to/<code>`
2. Backend responds with **HTTP 402** including x402 challenge and payment details (price, recipient, facilitator)
3. Wallet (e.g., Phantom) prompts for payment in SOL
4. On-chain confirmation is observed/validated; backend issues a short-lived, single-use redirect token
5. Visitor is redirected to the destination URL

---

## Architecture

- **Client**: Vite + React, Solana wallet adapter (Phantom), x402 client integration
- **Server**: Rust (e.g., Axum/Actix), MongoDB, x402 validation + link/payment services
- **Blockchain**: Solana devnet; SOL payments; optional integration with a configurable x402 facilitator

---

## Data Model (MVP)

### Link

```json
{
  "_id": "ObjectId",
  "code": "string",               // short code e.g., abc123
  "creatorPubkey": "string",      // base58
  "recipientPubkey": "string",    // base58 (defaults to creatorPubkey)
  "targetUrl": "string",
  "priceLamports": 123456789,      // price in lamports
  "feeBps": 300,                   // 3% platform fee
  "createdAt": "ISO-8601"
}
```

---

## Private API (MVP)

Base URL (dev): `http://localhost:4000` (server), `http://localhost:3000` (client)

Note: The API is private during the MVP. It is intended to be consumed only by the Hatchr client. Endpoints and contracts may change without notice and are not meant for third-party integrations yet.

### Create Link

POST `/api/links`

```bash
curl -X POST http://localhost:4000/api/links \
  -H "Content-Type: application/json" \
  -d '{
    "targetUrl": "https://example.com/my-course",
    "priceSol": 0.05,
    "recipientPubkey": "<optional base58>"
  }'
```

Response

```json
{
  "code": "abc123",
  "shortUrl": "https://hatchr.xyz/l/abc123",
  "priceSol": 0.05
}
```

### List My Links

GET `/api/links`

### Get Stats

GET `/api/links/:code/stats`

### Resolve Short Link (x402)

GET `/l/:code` → returns HTTP 402 x402 challenge when unpaid, then redirects upon successful payment. Public-facing short links resolve under `https://hatchr.xyz/l/:code`.

---

## Configuration

### Prerequisites

- Node.js 18+
- Rust stable (toolchain) and Cargo
- MongoDB (local or managed)

### Environment Variables

Server (`.env`):

```bash
PORT=4000
MONGODB_URI=mongodb://localhost:27017/hatchr
SOLANA_CLUSTER=devnet
X402_FACILITATOR_URL=https://facilitator.devnet.example.com
HATCHR_TREASURY_PUBKEY=<hatchr_treasury_pubkey_base58>
HATCHR_FEE_BPS=300
SESSION_SECRET=change_me
```

Client (`.env`):

```bash
VITE_BACKEND_URL=http://localhost:4000
VITE_SOLANA_CLUSTER=devnet
VITE_X402_FACILITATOR_URL=https://facilitator.devnet.example.com
```

#### Local Ports (dev)

- Server API: `http://localhost:4000` (configure with `PORT`)
- Client dev server: `http://localhost:3000` (run Vite with `--port 3000`)
- MongoDB: `mongodb://localhost:27017`

---

## Local Development

> Repo structure (planned):

```
hatchr/
  server/          # Rust (Axum/Actix), x402 verification, MongoDB
  client/          # Vite + React, wallet adapter, UI
  README.md
```

### Server

```bash
cd server
cargo run
```

### Client

```bash
cd client
npm install
npm run dev
```

---

## Payment Flow Details (x402)

- Hatchr issues an HTTP 402 challenge with all payment parameters required by an x402-compatible wallet/agent.
- After on-chain confirmation, Hatchr verifies the transaction (amount, recipient, recentness, and uniqueness) and issues a signed, single-use redirect token.
- The visitor is then redirected to the destination URL; stats are recorded and fees allocated.

---

## Hackathon Compliance

- Open source codebase (MIT license)
- Integrates x402 protocol and deploys on Solana devnet for demo
- Demo video ≤ 3 minutes showing wallet paywall and redirect
- Documentation includes quickstart and integration notes

Event details: [Solana X402 Hackathon](https://solana.com/x402/hackathon)

---

## Roadmap to Submission (Nov 3 → Nov 11)

- Nov 3–4: Scaffold backend (Rust + MongoDB), link model, x402 challenge endpoint
- Nov 4–6: Frontend (Vite/React), wallet connect, create-link form, list links
- Nov 6–7: x402 verification & redirect tokens, SOL payments end-to-end on devnet
- Nov 8–9: Stats/analytics, edge cases, rate limiting, polish
- Nov 10: Demo video, README cleanup, devnet deployment
- Nov 11: Submission

---

## Contributing

PRs welcome. Please open issues for bugs and feature requests.

---

## License

MIT
