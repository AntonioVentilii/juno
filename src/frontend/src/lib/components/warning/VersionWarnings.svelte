<script lang="ts">
	import { isNullish, nonNullish } from '@dfinity/utils';
	import type { BuildType } from '@junobuild/admin';
	import { compare } from 'semver';
	import type { Satellite } from '$declarations/mission_control/mission_control.did';
	import VersionWarning from '$lib/components/warning/VersionWarning.svelte';
	import { missionControlVersion } from '$lib/derived/version.derived';
	import { newerReleases } from '$lib/services/upgrade.services';
	import { busy } from '$lib/stores/busy.store';
	import { i18n } from '$lib/stores/i18n.store';
	import { versionStore } from '$lib/stores/version.store';
	import { emit } from '$lib/utils/events.utils';

	interface Props {
		satellite?: Satellite | undefined;
	}

	let { satellite = undefined }: Props = $props();

	let satVersion: string | undefined = $derived(
		nonNullish(satellite) && nonNullish(satellite?.satellite_id)
			? $versionStore?.satellites[satellite?.satellite_id.toText()]?.current
			: undefined
	);
	let satRelease: string | undefined = $derived(
		nonNullish(satellite) && nonNullish(satellite?.satellite_id)
			? $versionStore?.satellites[satellite?.satellite_id.toText()]?.release
			: undefined
	);
	let satBuild: BuildType | undefined = $derived(
		nonNullish(satellite) && nonNullish(satellite?.satellite_id)
			? $versionStore?.satellites[satellite?.satellite_id.toText()]?.build
			: undefined
	);

	let ctrlVersion: string | undefined = $derived($missionControlVersion?.current);
	let ctrlRelease: string | undefined = $derived($missionControlVersion?.release);

	let orbVersion: string | undefined = $derived($versionStore?.orbiter?.current);
	let orbRelease: string | undefined = $derived($versionStore?.orbiter?.release);

	let satReady = $derived(
		nonNullish($versionStore) && nonNullish(satVersion) && nonNullish(satRelease)
	);

	let ctrlReady = $derived(
		nonNullish($versionStore) && nonNullish(ctrlVersion) && nonNullish(ctrlRelease)
	);

	let orbReady = $derived(
		nonNullish($versionStore) && nonNullish(orbVersion) && nonNullish(orbRelease)
	);

	let satWarning = $derived(
		nonNullish(satVersion) && nonNullish(satRelease) && compare(satVersion, satRelease) < 0
	);

	let ctrlWarning = $derived(
		nonNullish(ctrlVersion) && nonNullish(ctrlRelease) && compare(ctrlVersion, ctrlRelease) < 0
	);

	let orbWarning = $derived(
		nonNullish(orbVersion) && nonNullish(orbRelease) && compare(orbVersion, orbRelease) < 0
	);

	const openModal = async ({
		currentVersion,
		type,
		satellite,
		build
	}: {
		currentVersion: string;
		type: 'upgrade_satellite' | 'upgrade_mission_control' | 'upgrade_orbiter';
		satellite?: Satellite;
		build?: BuildType;
	}) => {
		busy.start();

		const { result, error } = await newerReleases({
			currentVersion,
			segments:
				type === 'upgrade_mission_control'
					? 'mission_controls'
					: type === 'upgrade_orbiter'
						? 'orbiters'
						: 'satellites'
		});

		busy.stop();

		if (nonNullish(error) || isNullish(result)) {
			return;
		}

		emit({
			message: 'junoModal',
			detail: {
				type,
				detail: {
					...(nonNullish(satellite) && { satellite }),
					currentVersion,
					newerReleases: result,
					build
				}
			}
		});
	};

	const upgradeSatellite = async () =>
		await openModal({
			type: 'upgrade_satellite',
			satellite: satellite!,
			currentVersion: satVersion!,
			build: satBuild
		});

	const upgradeMissionControl = async () =>
		await openModal({
			type: 'upgrade_mission_control',
			currentVersion: ctrlVersion!
		});

	const upgradeOrbiter = async () =>
		await openModal({
			type: 'upgrade_orbiter',
			currentVersion: orbVersion!
		});
</script>

{#if ctrlReady && ctrlWarning}
	<VersionWarning text={$i18n.admin.mission_control_new_version} onclick={upgradeMissionControl} />
{/if}

{#if orbReady && orbWarning}
	<VersionWarning text={$i18n.admin.orbiter_new_version} onclick={upgradeOrbiter} />
{/if}

{#if satReady && satWarning}
	<VersionWarning text={$i18n.admin.satellite_new_version} onclick={upgradeSatellite} />
{/if}
