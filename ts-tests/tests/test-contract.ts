import { expect } from "chai";
import testContract from "../build/contracts/test.json"
import { createAndFinalizeBlock, customRequest, describeWithFrontier } from "./util";

describeWithFrontier("Frontier RPC (Contract)", `simple-specs.json`, (context) => {
	const GENESIS_ACCOUNT = "0x6be02d1d3665660d22ff9624b7be0551ee1ac91b";
	const GENESIS_ACCOUNT_PRIVATE_KEY = "0x99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342";

	const TEST_CONTRACT_BYTECODE = testContract.bytecode;
	const FIRST_CONTRACT_ADDRESS = "0xc2bf5f29a4384b1ab0c063e1c666f02121b6084a";
	// Those test are ordered. In general this should be avoided, but due to the time it takes
	// to spin up a frontier node, it saves a lot of time.

	it("contract creation should return transaction hash", async function () {
		this.timeout(15000);
		const tx = await context.web3.eth.accounts.signTransaction(
			{
				from: GENESIS_ACCOUNT,
				data: TEST_CONTRACT_BYTECODE,
				value: "0x00",
				gasPrice: "0x01",
				gas: "0x100000",
			},
			GENESIS_ACCOUNT_PRIVATE_KEY
		);

		expect(await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction])).to.deep.equal({
			id: 1,
			jsonrpc: "2.0",
			result: "0x7f7055775146c3f13e0dcee82375aedb3c71a24a021f63921bc5c7216040a273",
		});

		// Verify the contract is not yet stored
		expect(await customRequest(context.web3, "eth_getCode", [FIRST_CONTRACT_ADDRESS])).to.deep.equal({
			id: 1,
			jsonrpc: "2.0",
			result: "0x",
		});

		// Verify the contract is stored after the block is produced
		await createAndFinalizeBlock(context.web3);
		expect(await customRequest(context.web3, "eth_getCode", [FIRST_CONTRACT_ADDRESS])).to.deep.equal({
			id: 1,
			jsonrpc: "2.0",
			result:
				"0x6080604052348015600f57600080fd5b506004361060285760003560e01c8063c6888fa114602d575b600080fd5b605660048036036020811015604157600080fd5b8101908080359060200190929190505050606c565b6040518082815260200191505060405180910390f35b600060078202905091905056fea265627a7a72315820a304e718a572041343f0fba6a040c7674c0992956b1c8caa40482fe0b6d6499764736f6c63430005100032",
		});
	});
});
