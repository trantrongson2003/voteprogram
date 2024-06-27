import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import type { VoteProgram } from "../target/types/vote_program";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.VoteProgram as anchor.Program<VoteProgram>;


async function main() {
  try {
    const networkUrl = "https://api.devnet.solana.com"; // hoặc mạng khác
    const connection = new Connection(networkUrl, "confirmed");

    // Tạo một keypair cho ví
    const wallet = Keypair.generate();

    console.log("My address:", wallet.publicKey.toString());

    // Lấy số dư của ví
    const balance = await connection.getBalance(wallet.publicKey);
    console.log(`My balance: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);

    // Tiếp tục với tương tác với chương trình Anchor ở đây
  } catch (error) {
    console.error("Error:", error);
  }
}

main()
  .then(() => console.log("Success"))
  .catch((err) => console.error(err));
