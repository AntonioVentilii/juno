import type { UpgradeCodeProgressState } from '@junobuild/admin';

export enum TopUpProgressStep {
	TopUp = 0,
	Reload = 1
}

export type TopUpProgressState = UpgradeCodeProgressState;

export interface TopUpProgress {
	step: TopUpProgressStep;
	state: TopUpProgressState;
}
