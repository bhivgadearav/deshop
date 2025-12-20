## DeShop: Decentralized Shop
DeShop is shopify for digital products but going even further to make receiving payments even more accessible using CINR (Crypto Indian Rupees) which is cryptocurrency on the Solana blockchain pegged to real INR.

### Modules
1. CINR (Crypto Indian Rupees) Payments - Customers can pay the sellers using CINR one-time or on a subscription basis. Sellers receive payments on their assigned PDA address. Sellers can claim their accumulated revenue from deshop's solana program every month but deshop will tak 10% fees on every claim.

2. Escrow + Conditional Release - Payment goes into a program-controlled PDA escrow; funds are released to seller when buyer confirms delivery or after a timeout. Disputes can be escalated.

3. Store Management - Sellers can add products individually or in bulk in csv format.

4. Store Customization - Sellers can customize their stores in various ways like theme, logo, font etc.

5. Store Discovery - Customers can discover sellers and their stores. Compare similar products between sellers to make a responsible decision before purchasing.

6. Product Licensing/Access Control - Whenever a customer buys a product a relevant NFT is minted to his wallet which is the proof of his purchase. This NFT is used for accessing/downloading/using the product he purchased.

7. Decentralized Dispute Resolution - Buyers can open disputes; a governance token (DESHOP GOV) holder vote resolves it and authorizes refund.

### Tech Stack - TypeScript Focused
1. Package Manager - Bun
2. Codebase - Turborepo
3. Frontend - Next.js, Tailwind CSS, lucide-react, shadcn/ui, Motion For React, Tanstack React Query, Jotai, Zod, Axios
4. Backend - Express.js, Zod, Prisma, Passport.js, tRPC, ioredis, express-rate-limit, cors, helmet
5. Database - PostgreSQL
6. Caching - Redis
7. Indexer - TypeScript
8. Blockchain - @solana/web3.js, anza-xyz/wallet-adapter
9. Solana Program - Rust, Anchor
10. Containerization - Docker
11. CI/CD - GitHub Actions
12. Linting & Formatting — ESLint + Prettier + TypeScript strictness rules
