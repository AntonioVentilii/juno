import {
	defineHook,
	type OnSetDoc,
	type OnSetDocContext,
	type RunFunction
} from '@junobuild/functions';
import { testIcCdkCall } from './apis/ic-cdk/ic-cdk-call';
import { testIcCdkId } from './apis/ic-cdk/ic-cdk-id';
import { testIcCdkTime } from './apis/ic-cdk/ic-cdk-time';
import { testBlob } from './apis/node/blob';
import { testMathRandom } from './apis/node/math';
import { testTextEncoding } from './apis/node/text-encoding';
import {
	testSdkCountAssetsStore,
	testSdkCountCollectionAssetsStore,
	testSdkDeleteAssetsStore,
	testSdkDeleteAssetStore,
	testSdkDeleteFilteredAssetsStore,
	testSdkGetAssetStore,
	testSdkGetContentChunksStore,
	testSdkListAssetsStore,
	testSdkSetAssetHandler
} from './sdk/assets';
import { testSdkControllers } from './sdk/controllers';
import {
	testSdkCountCollectionDocsStore,
	testSdkCountDocsStore,
	testSdkDeleteDocsStore,
	testSdkDeleteDocStore,
	testSdkDeleteFilteredDocsStore,
	testSdkGetDocStore,
	testSdkListDocsStore,
	testSdkSetDocStore
} from './sdk/docs';

/* eslint-disable require-await, no-console */

const testSetDoc = async (context: OnSetDocContext) => {
	console.log('onSetDoc:', context.data.key);
};

/* eslint-enable */

const collections = [
	'test-onsetdoc',
	'test-ic-cdk-id',
	'test-ic-cdk-call',
	'test-ic-cdk-time',
	'test-update',
	'test-textencoding',
	'test-mathrandom',
	'test-blob',
	'test-deletedoc',
	'test-getdoc',
	'test-sdk-controllers',
	'test-listdocs',
	'test-countcollectiondocs',
	'test-countdocs',
	'test-countcollectionassets',
	'test-countassets',
	'test-deletedocs',
	'test-deletefiltereddocs',
	'test-setassethandler',
	'test-deleteasset',
	'test-deleteassets',
	'test-deletefilteredassets',
	'test-getasset',
	'test-listassets',
	'test-getchunks'
] as const;

type OnSetDocCollection = (typeof collections)[number];

export const onSetDoc = defineHook<OnSetDoc>({
	collections,
	run: async (context) => {
		const fn: Record<OnSetDocCollection, RunFunction<OnSetDocContext>> = {
			'test-onsetdoc': testSetDoc,
			'test-ic-cdk-id': testIcCdkId,
			'test-ic-cdk-time': testIcCdkTime,
			'test-update': testSdkSetDocStore,
			'test-ic-cdk-call': testIcCdkCall,
			'test-textencoding': testTextEncoding,
			'test-mathrandom': testMathRandom,
			'test-blob': testBlob,
			'test-deletedoc': testSdkDeleteDocStore,
			'test-getdoc': testSdkGetDocStore,
			'test-listdocs': testSdkListDocsStore,
			'test-sdk-controllers': testSdkControllers,
			'test-countcollectiondocs': testSdkCountCollectionDocsStore,
			'test-countdocs': testSdkCountDocsStore,
			'test-countcollectionassets': testSdkCountCollectionAssetsStore,
			'test-countassets': testSdkCountAssetsStore,
			'test-deletedocs': testSdkDeleteDocsStore,
			'test-deletefiltereddocs': testSdkDeleteFilteredDocsStore,
			'test-setassethandler': testSdkSetAssetHandler,
			'test-deleteasset': testSdkDeleteAssetStore,
			'test-deleteassets': testSdkDeleteAssetsStore,
			'test-deletefilteredassets': testSdkDeleteFilteredAssetsStore,
			'test-getasset': testSdkGetAssetStore,
			'test-listassets': testSdkListAssetsStore,
			'test-getchunks': testSdkGetContentChunksStore
		};

		await fn[context.data.collection as OnSetDocCollection]?.(context);
	}
});
