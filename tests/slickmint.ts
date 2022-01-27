import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Slickmint } from '../target/types/slickmint';

describe('slickmint', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Slickmint as Program<Slickmint>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
