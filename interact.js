const { ethers } = require("ethers");

const rpcUrl = "https://sepolia-rpc.kakarot.org";
const privateKey = "ed8824752e695757b46c5a4a27ece67e406da440feb28c451f419a0718756b17";
const contractAddress = "0x4c1fF993E16b493aEC456117d1B515567118188e";

// Füge das generierte ABI hier ein
const abi = [
    {
        "inputs": [],
        "name": "performInteraction",
        "outputs": [],
        "stateMutability": "payable",
        "type": "function"
    }
];

async function main() {
    const provider = new ethers.providers.JsonRpcProvider(rpcUrl);
    const wallet = new ethers.Wallet(privateKey, provider);
    const contract = new ethers.Contract(contractAddress, abi, wallet);

    const value = ethers.parseEther("0.00001");
    const tx = await contract.performInteraction({ value });
    console.log("Transaction hash:", tx.hash);

    const receipt = await tx.wait();
    console.log("Transaction was mined in block:", receipt.blockNumber);
}

main().catch((error) => {
    console.error("Error:", error);
});

