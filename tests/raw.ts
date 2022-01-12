import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Raw } from '../target/types/raw';

describe('raw', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Raw as Program<Raw>;

  it('Is initialized!', async () => {

  });
});
