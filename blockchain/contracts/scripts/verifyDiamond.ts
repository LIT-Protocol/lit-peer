import { Contract } from 'ethers';
import hre from 'hardhat';
import { FacetCutAction } from './diamondContracts/lib/types';
import { getSelectors } from './diamondContracts/lib/utils';
import { Environment } from '../utils/contract';
import { verifyContractInBg } from './utils';
const { ethers } = hre;

export async function verifyDiamond(
  diamondContractName: string,
  diamondContractAddress: string,
  diamondInitContractAddress: string,
  contractResolverAddress: string,
  env: Environment,
  diamondOwnerAddress: string,
  facetAddressMap: FacetAddressMap,
  {
    additionalFacets = [],
    useErc165Loupe = false,
  }: {
    additionalFacets?: string[];
    useErc165Loupe?: boolean;
  }
) {
  console.log('Verifying diamond...');
  const diamondInit = await ethers.getContractAt(
    'DiamondInit',
    diamondInitContractAddress
  );

  const FacetNames = [
    'DiamondCutFacet',
    useErc165Loupe ? 'DiamondLoupeFacet' : 'DiamondLoupeFacetNoERC165',
    'OwnershipFacet',
    ...additionalFacets,
  ];

  const facetCuts = [];
  for (const FacetName of FacetNames) {
    // Get the facet contract at the address.
    if (!facetAddressMap[FacetName]) {
      throw new Error(
        `Facet address not found for ${FacetName}. Please provide the address in the facetAddressMap.`
      );
    }

    const facet = await ethers.getContractAt(
      FacetName,
      facetAddressMap[FacetName]
    );
    const facetAddress = await facet.getAddress();
    facetCuts.push({
      facetAddress: facetAddress,
      action: FacetCutAction.Add,
      functionSelectors: getSelectors(facet).map((s) => s.selector),
    });
  }

  const functionCall = diamondInit.interface.encodeFunctionData('init');

  // Setting arguments that will be used in the diamond constructor
  const diamondArgs: any = {
    owner: diamondOwnerAddress,
    init: await diamondInit.getAddress(),
    initCalldata: functionCall,
    contractResolver: contractResolverAddress,
    env,
  };

  // Verify the diamond contract
  await verifyContractInBg(diamondContractName, diamondContractAddress, [
    facetCuts,
    diamondArgs,
  ]);

  console.log('facetCuts: ', facetCuts);
}

interface FacetAddressMap {
  [key: string]: string;
}
