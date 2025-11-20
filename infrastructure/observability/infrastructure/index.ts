import * as gcp from '@pulumi/gcp';
import { Account } from '@pulumi/gcp/serviceaccount/account';
import { cloudrunv2 } from '@pulumi/gcp/types/input';
import * as pulumi from '@pulumi/pulumi';
import { Network, Node } from './constants';
import * as fs from 'fs';
import * as path from 'path';
import * as crypto from 'crypto';
import { env } from 'process';
import { createEpochDelayMonitor } from './monitors/epoch-delay-monitor';
import { createReplicaOutOfSyncMonitor } from './monitors/replica-out-of-sync-monitor';

const LOCATION = 'us-west2';
const REPOSITORY_NAME = 'observability';

// Docker image URLs for observability probes
const CHAIN_PROBE_IMAGE = `${LOCATION}-docker.pkg.dev/${gcp.config.project!}/${REPOSITORY_NAME}/chain-probe-image:latest`;
const NETWORK_PROBE_IMAGE = `${LOCATION}-docker.pkg.dev/${gcp.config.project!}/${REPOSITORY_NAME}/network-probe-image:latest`;

const litConfig = new pulumi.Config('lit');
const pkpContractAddress = litConfig.require('contracts-pkp');
const stakingContractAddress = litConfig.require('contracts-staking');
const chainName = litConfig.require('chain-name');
const networkProbePrivateKey = litConfig.requireSecret(
  'network-probe-private-key'
);

const PROBE_INPUTS: ProbeInputMap = {
  numberOfPkps: {
    contractAddress: pkpContractAddress,
  },
  validatorsInCurrentEpoch: {
    contractAddress: stakingContractAddress,
  },
  delayToAdvancingNextEpoch: {
    contractAddress: stakingContractAddress,
  },
  kickedValidatorsInNextEpoch: {
    contractAddress: stakingContractAddress,
  },
};

const NODES_BY_NETWORK: { [key in Network]: Node[] } = {
  [Network.Internal]: [
    Node.OvhStaging4,
    Node.OvhStaging5,
    Node.OvhStaging6,
    Node.OvhStaging7,
    Node.OvhStaging8,
    Node.LeasewebStaging5,
    Node.LeasewebStaging6,
    Node.LeasewebStaging7,
  ],
  [Network.DatilDev]: [Node.Gateway],
  [Network.DatilTest]: [
    Node.OvhStaging4,
    Node.OvhStaging5,
    Node.LeasewebStaging4,
    Node.LeasewebManzano1,
    Node.Orbis1,
    Node.ZerionTest,
    Node.Everstake,
  ],
  [Network.DatilProd]: [
    Node.LeasewebHabanero1,
    Node.HyphaMain,
    Node.Cheqd,
    Node.Imperator,
    Node.ZeroOneNodes,
    Node.ZerionMain,
    Node.Thunderhead,
    Node.HirenodesMain,
  ],
  [Network.NagaDev]: [Node.Gateway],
  [Network.NagaStaging]: [
    Node.CherryserverKeyMoose,
    Node.LeasewebM64_1,
    Node.LeasewebM64_2,
    Node.LeasewebM64_3,
    Node.OvhQaLoadTest1,
    Node.DedicatedM64Garnet,
    Node.DedicatedM64Hope,
    Node.DedicatedM64Irvine,
    Node.DedicatedM64Jesse,
    Node.DedicatedM64Kain,
  ],
  [Network.NagaTest]: [
    Node.LeasewebStaging8,
    Node.DedicatedM24Fork,
    Node.Patch1,
    Node.SignM64,
    Node.HyphaM24,
    Node.Terminal3,
  ],
};

// TODO: These all need to be set to proper pubkeys.
const PUBLIC_KEYS_BY_PATH: { [key in Node]: string } = {
  // internal
  [Node.Gateway]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebStaging3]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebStaging4]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebStaging5]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebStaging6]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebStaging7]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebStaging8]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebManzano1]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebHabanero1]: './shared_internaldev_gpg_pubkey.asc',
  [Node.OvhStaging4]: './shared_internaldev_gpg_pubkey.asc',
  [Node.OvhStaging5]: './shared_internaldev_gpg_pubkey.asc',
  [Node.OvhStaging6]: './shared_internaldev_gpg_pubkey.asc',
  [Node.OvhStaging7]: './shared_internaldev_gpg_pubkey.asc',
  [Node.OvhStaging8]: './shared_internaldev_gpg_pubkey.asc',

  // external
  [Node.LeasewebTerminal3]:
    './keys/A9BB48AF2C2EF659240F1FAC5C7DE5FA7457F85C.asc',
  [Node.LeasewebCollabland]:
    './keys/8A6C80597DE066BB3660A643FCA1D6FA4A74F71F.asc',
  [Node.LeasewebStreamr]: './keys/E40D2583A7061CFECF1EA2FD3269EE2C55717D8F.asc',
  [Node.OvhBwareLabs]: './keys/A75890D5D0651825B563778BBBD9B139EE8C4C4F.asc',
  [Node.Leaseweb1kx]: './keys/2C78BDE49E3325F1221E4AD0A0245128E4836A8B.asc',
  [Node.LeasewebMolecule]:
    './keys/EAC8E99CC45D2941CBC718FEB326D0BD142DE65E.asc',
  [Node.OvhImperator]: './keys/42E7EB9EF0F1EAE9760538B9542E415688A35400.asc',
  [Node.Leaseweb01node]: './keys/6BCB3DC2EF68653946D61B07D065E84627A56567.asc',
  [Node.LeasewebCmtDigital]:
    './keys/524FE383743031E867F87992694E3C6C4409F797.asc',
  [Node.LeasewebPatch]: './keys/3556930ACDA56CDA95510C89FEDE10DEEC82C58D.asc',
  [Node.LeasewebHyphaCoop]:
    './keys/AEC7732D41BBF2498C8582188E658C61734B3A4B.asc',
  [Node.LeasewebOrbis]: './keys/8280E4D0167799CAE6F0E0FAB05E94D069ED993D.asc',

  [Node.HyphaMain]: './shared_internaldev_gpg_pubkey.asc',
  [Node.Cheqd]: './shared_internaldev_gpg_pubkey.asc',
  [Node.Imperator]: './shared_internaldev_gpg_pubkey.asc',
  [Node.ZeroOneNodes]: './shared_internaldev_gpg_pubkey.asc',
  [Node.ZerionMain]: './shared_internaldev_gpg_pubkey.asc',
  [Node.Thunderhead]: './shared_internaldev_gpg_pubkey.asc',
  [Node.HirenodesMain]: './shared_internaldev_gpg_pubkey.asc',
  [Node.Everstake]: './shared_internaldev_gpg_pubkey.asc',
  [Node.ZerionTest]: './shared_internaldev_gpg_pubkey.asc',
  [Node.Orbis1]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebLit1]: './shared_internaldev_gpg_pubkey.asc',

  // naga-staging
  [Node.CherryserverKeyMoose]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebM64_1]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebM64_2]: './shared_internaldev_gpg_pubkey.asc',
  [Node.LeasewebM64_3]: './shared_internaldev_gpg_pubkey.asc',
  [Node.OvhQaLoadTest1]: './shared_internaldev_gpg_pubkey.asc',
  [Node.DedicatedM64Garnet]: './shared_internaldev_gpg_pubkey.asc',
  [Node.DedicatedM64Hope]: './shared_internaldev_gpg_pubkey.asc',
  [Node.DedicatedM64Irvine]: './shared_internaldev_gpg_pubkey.asc',
  [Node.DedicatedM64Jesse]: './shared_internaldev_gpg_pubkey.asc',
  [Node.DedicatedM64Kain]: './shared_internaldev_gpg_pubkey.asc',

  // naga-test
  [Node.DedicatedM24Fork]: './shared_internaldev_gpg_pubkey.asc',
  [Node.Patch1]: './shared_internaldev_gpg_pubkey.asc',
  [Node.SignM64]: './shared_internaldev_gpg_pubkey.asc',
  [Node.HyphaM24]: './shared_internaldev_gpg_pubkey.asc',
  [Node.Terminal3]: './shared_internaldev_gpg_pubkey.asc',
};

interface ProbeInputMap {
  [key: string]: {
    contractAddress: string;
  };
}

// Get the existing Google Artifact Registry repository created by the workflow
const repository = gcp.artifactregistry.getRepository({
  repositoryId: REPOSITORY_NAME,
  location: LOCATION,
  project: gcp.config.project!,
});

// Create service accounts (will be created if they don't exist, ignored if they do)
const cloudSchedulerServiceAccount = new gcp.serviceaccount.Account(
  'cloud_scheduler_service_account',
  {
    accountId: 'cloud-scheduler-sa',
    displayName: 'Cloud Scheduler Service Account',
    description: 'Pulumi Managed service account for the Cloud Scheduler Jobs',
    createIgnoreAlreadyExists: true,
  }
);

const cloudRunServiceAccount = new gcp.serviceaccount.Account(
  'cloud_run_service_account',
  {
    accountId: 'cloud-run-sa',
    displayName: 'Cloud Run Service Account',
    description: 'Pulumi Managed service account for the Cloud Run Jobs',
    createIgnoreAlreadyExists: true,
  }
);

setupPermissionsForCloudRunJob(cloudRunServiceAccount);

// Create the secret if it doesn't exist.
const networkProbePrivateKeySecret = pulumi
  .output(gcp.organizations.getProject({}))
  .apply((p) =>
    gcp.secretmanager
      .getSecret(
        {
          secretId: 'network-probe-private-key',
          project: p.projectId,
        },
        { async: true }
      )
      .then((s) =>
        gcp.secretmanager.Secret.get('network-probe-private-key', s.id)
      )
      .catch((err) => {
        return new gcp.secretmanager.Secret('network-probe-private-key', {
          secretId: 'network-probe-private-key',
          replication: {
            auto: {},
          },
        });
      })
  );

// Create a secret version with secret data from the Pulumi config.
const networkProbePrivateKeySecretVersion = new gcp.secretmanager.SecretVersion(
  'networkProbePrivateKeyVersion',
  {
    secret: networkProbePrivateKeySecret.id,
    secretData: networkProbePrivateKey,
  }
);

// Schedule the Chain Probes
createScheduledProbe(
  'numberOfPkps',
  getEnvsForChainProbe('numberOfPkps', PROBE_INPUTS),
  {
    imageUrl: CHAIN_PROBE_IMAGE,
    cloudSchedulerServiceAccount,
    cloudRunServiceAccount,
  }
);

createScheduledProbe(
  'validatorsInCurrentEpoch',
  getEnvsForChainProbe('validatorsInCurrentEpoch', PROBE_INPUTS),
  {
    imageUrl: CHAIN_PROBE_IMAGE,
    cloudSchedulerServiceAccount,
    cloudRunServiceAccount,
  }
);

createScheduledProbe(
  'delayToAdvancingNextEpoch',
  getEnvsForChainProbe('delayToAdvancingNextEpoch', PROBE_INPUTS),
  {
    imageUrl: CHAIN_PROBE_IMAGE,
    cloudSchedulerServiceAccount,
    cloudRunServiceAccount,
  }
);

createScheduledProbe(
  'kickedValidatorsInNextEpoch',
  getEnvsForChainProbe('kickedValidatorsInNextEpoch', PROBE_INPUTS),
  {
    imageUrl: CHAIN_PROBE_IMAGE,
    cloudSchedulerServiceAccount,
    cloudRunServiceAccount,
  }
);

// Get webhook integration details from IRM stack
const irmStack = new pulumi.StackReference("lit-protocol-devs/irm-infrastructure/dev");
const webhookIntegrationUrl = irmStack.getOutput("gcpWebhookIntegrationUrl");
const webhookIntegrationToken = irmStack.getOutput("gcpWebhookIntegrationToken");

// Create base notification channel for all alerts
const baseNotificationChannel = new gcp.monitoring.NotificationChannel("base-webhook", {
  displayName: "Base Webhook Channel",
  description: "Notification channel for Alerts to Grafana IRM",
  type: "webhook_tokenauth",
  labels: {
    url: pulumi.interpolate`${webhookIntegrationUrl}?auth_token=${webhookIntegrationToken}`,
  },
});

export const baseWebhookChannel = baseNotificationChannel;
// Create the epoch delay GCP monitor
const epochMonitor = createEpochDelayMonitor();

// Create the replica out of sync monitor for `naga` environments only
if (pulumi.getStack().startsWith('naga-')) {
  createReplicaOutOfSyncMonitor();
}

// Only create the network probe for the following stacks:
// - datil-dev, datil-test, datil-prod
// - naga-dev, naga-staging, naga-test
if (
  pulumi.getStack() === 'datil-dev' ||
  pulumi.getStack() === 'datil-test' ||
  pulumi.getStack() === 'datil-prod' ||
  pulumi.getStack() === 'naga-dev' ||
  pulumi.getStack() === 'naga-staging' ||
  pulumi.getStack() === 'naga-test'
) {
  // Schedule the Network Probe
  createScheduledProbe('executeJs', getEnvsForNetworkProbe('executeJs'), {
    imageUrl: NETWORK_PROBE_IMAGE,
    cloudSchedulerServiceAccount,
    cloudRunServiceAccount,
  });
}

function getEnvsForChainProbe(
  probeName: string,
  probeInputMap: ProbeInputMap
): cloudrunv2.JobTemplateTemplateContainerEnv[] {
  return [
    {
      name: 'PROBE_NAME',
      value: probeName,
    },
    {
      name: 'CONTRACT_ADDRESS',
      value: probeInputMap[probeName].contractAddress,
    },
    {
      name: 'GCP_PROJECT_ID',
      value: gcp.config.project!,
    },
    {
      name: 'CHAIN_NAME',
      value: chainName,
    },
  ];
}

function getEnvsForNetworkProbe(
  probeName: string
): cloudrunv2.JobTemplateTemplateContainerEnv[] {
  return [
    {
      name: 'PROBE_NAME',
      value: probeName,
    },
    {
      name: 'GCP_PROJECT_ID',
      value: gcp.config.project!,
    },
    {
      name: 'CHAIN_NAME',
      value: chainName,
    },
  ];
}

// Schedule the function to run every 1 minute.
function createScheduledProbe(
  probeName: string,
  envs: cloudrunv2.JobTemplateTemplateContainerEnv[],
  options: Options
) {
  // Deploy to Cloud Run as a job.
  const cloudRunJob = new gcp.cloudrunv2.Job(`${probeName}-job`.toLowerCase(), {
    location: LOCATION,
    template: {
      template: {
        serviceAccount: options.cloudRunServiceAccount.email,
        containers: [
          {
            image: options.imageUrl,
            resources: {
              limits: {
                memory: '512Mi',
              },
            },
            envs,
          },
        ],
      },
    },
  });

  // Grant the `roles/run.invoker` role to the Cloud Scheduler service account.
  const invokerBinding = new gcp.cloudrunv2.JobIamMember(
    `${probeName}-invoker-binding`,
    {
      name: cloudRunJob.name,
      location: cloudRunJob.location,
      role: 'roles/run.invoker',
      member: pulumi.interpolate`serviceAccount:${options.cloudSchedulerServiceAccount.email}`,
    },
    {
      dependsOn: [options.cloudSchedulerServiceAccount],
    }
  );

  const schedulerJob = new gcp.cloudscheduler.Job(
    `${probeName}-schedule`,
    {
      name: `${probeName}-schedule`,
      description: `Schedule for ${probeName} probe`,
      schedule: '* * * * *',
      timeZone: 'UTC',
      region: LOCATION,
      attemptDeadline: '320s',

      httpTarget: {
        httpMethod: 'POST',
        uri: pulumi.interpolate`https://${LOCATION}-run.googleapis.com/v2/projects/${gcp.config.project}/locations/${LOCATION}/jobs/${cloudRunJob.name}:run`,
        oauthToken: {
          serviceAccountEmail: options.cloudSchedulerServiceAccount.email,
        },
      },
    },
    {
      dependsOn: [invokerBinding, cloudRunJob],
    }
  );
}

function setupPermissionsForCloudRunJob(cloudRunServiceAccount: Account) {
  // Grant the Cloud Run Job's service account the role to publish metrics
  const metricWriterRole = new gcp.projects.IAMBinding(
    'metric-writer-role',
    {
      project: gcp.config.project!,
      role: 'roles/monitoring.metricWriter',
      members: [
        pulumi.interpolate`serviceAccount:${cloudRunServiceAccount.email}`,
      ],
    },
    {
      dependsOn: [cloudRunServiceAccount],
    }
  );

  // Grant the Cloud Run Job's service account the role to access secret manager
  const secretManagerAccessRole = new gcp.projects.IAMBinding(
    'secret-manager-access-role',
    {
      project: gcp.config.project!,
      role: 'roles/secretmanager.secretAccessor',
      members: [
        pulumi.interpolate`serviceAccount:${cloudRunServiceAccount.email}`,
      ],
    },
    {
      dependsOn: [cloudRunServiceAccount],
    }
  );
}

/**
 * Zips the given file paths into a single zip file.
 * @param filePaths - The file paths to zip.
 * @param destPath - The destination path for the zip file.
 * @returns The path to the zip file.
 */
function archiveFiles(filePaths: string[], destPath: string): string {
  const fileContents = filePaths.map((filePath) =>
    fs.readFileSync(filePath, 'utf-8')
  );
  const md5Hash = crypto
    .createHash('md5')
    .update(fileContents.join(' '))
    .digest('hex');

  const archivePath = path.join(destPath, `${md5Hash}.zip`);
  const zipCommand = `zip -j ${archivePath} ${fileContents.join(' ')}`;
  require('child_process').execSync(zipCommand);
  return archivePath;
}

function setupNodeOperatorLogging() {
  // Archive files using name as md5sum of the contents
  const tmpDir = require('os').tmpdir();
  const archivePath = archiveFiles(
    [
      path.resolve(__dirname, '../single-key-distributor/main.go'),
      path.resolve(__dirname, '../single-key-distributor/go.mod'),
      path.resolve(__dirname, '../single-key-distributor/go.sum'),
    ],
    tmpDir
  );

  // Create a Storage bucket for the Cloud Function's source archive
  const functionBucket = new gcp.storage.Bucket(
    `single-key-distributor-function-bucket`,
    {
      name: `single-key-distributor-function-bucket`,
      location: LOCATION,
      project: gcp.config.project!,
    }
  );

  // Create the archive of the Cloud Function's source code
  const archive = new gcp.storage.BucketObject(
    `single-key-distributor-function-archive`,
    {
      name: `single-key-distributor-function-archive`,
      bucket: functionBucket.name,
      source: new pulumi.asset.FileAsset(archivePath),
    }
  );

  // Create a service account for the distributor (context of the Cloud Function)
  const distributorServiceAccount = new gcp.serviceaccount.Account(
    'single-key-distributor',
    {
      accountId: 'single-key-distributor',
      project: gcp.config.project!,
      displayName: 'Single Key Distributor',
      description:
        'Pulumi Managed service account for the single key distributor',
    }
  );

  // Grant the distributor service account the serviceAccountKeyAdmin role.
  const serviceAccountKeyAdminRole = new gcp.projects.IAMBinding(
    `${distributorServiceAccount.email}-service-account-key-admin-role`,
    {
      project: gcp.config.project!,
      role: 'roles/iam.serviceAccountKeyAdmin',
      members: [
        pulumi.interpolate`serviceAccount:${distributorServiceAccount.email}`,
      ],
    }
  );

  // For each node in this network / stack, create a service account for the node logger.
  const nodesInNetwork = NODES_BY_NETWORK[stackToNetwork(pulumi.getStack())];
  for (const node of nodesInNetwork) {
    const nodeLoggerServiceAccount = new gcp.serviceaccount.Account(
      `${node}-logger`,
      {
        accountId: `${node}-logger`,
        project: gcp.config.project!,
        displayName: `${node} Logger`,
        description: `Pulumi Managed service account for the ${node} node logger`,
      }
    );

    // Grant the node logger service account the roles/logging.logWriter role.
    const logWriterRole = new gcp.projects.IAMBinding(
      `${node}-log-writer-role`,
      {
        project: gcp.config.project!,
        role: 'roles/logging.logWriter',
        members: [
          pulumi.interpolate`serviceAccount:${nodeLoggerServiceAccount.email}`,
        ],
      }
    );

    // Get the GPG Public Key for the node.
    const pubkey = fs.readFileSync(
      path.resolve(__dirname, PUBLIC_KEYS_BY_PATH[node]),
      'utf-8'
    );

    // Create the Cloud Function
    const cloudFunction = new gcp.cloudfunctions.Function(
      `${node}-single-key-distributor-function`,
      {
        project: gcp.config.project!,
        region: LOCATION,
        name: `${node}-single-key-distributor-function`,
        description:
          'Generates and encrypts a new Service Account key given a GPG public key',
        runtime: 'go121',
        triggerHttp: true,
        entryPoint: 'GenerateAndEncrypt',
        serviceAccountEmail: distributorServiceAccount.email,
        sourceArchiveBucket: functionBucket.name,
        sourceArchiveObject: archive.name,
        dockerRegistry: 'ARTIFACT_REGISTRY',
        dockerRepository: REPOSITORY_NAME,
        environmentVariables: {
          PUBLIC_KEY: pubkey,
          SERVICE_ACCOUNT_EMAIL_TARGET: nodeLoggerServiceAccount.email,
        },
      }
    );
  }
}

function stackToNetwork(stackName: string): Network {
  if (stackName.includes('internal')) {
    return Network.Internal;
  } else if (stackName.includes('datil-dev')) {
    return Network.DatilDev;
  } else if (stackName.includes('datil-test')) {
    return Network.DatilTest;
  } else if (stackName.includes('datil-prod')) {
    return Network.DatilProd;
  } else if (stackName.includes('naga-dev')) {
    return Network.NagaDev;
  } else if (stackName.includes('naga-staging')) {
    return Network.NagaStaging;
  } else if (stackName.includes('naga-test')) {
    return Network.NagaTest;
  } else {
    throw new Error(`Unknown stack name: ${stackName}`);
  }
}

interface Options {
  imageUrl: string;
  cloudSchedulerServiceAccount: Account;
  cloudRunServiceAccount: Account;
}

export const repositoryUrl = repository.then(repo => repo.id);
export const networkProbePrivateKeySecretName =
  networkProbePrivateKeySecret.name;
export const networkProbePrivateKeySecretVersionName =
  networkProbePrivateKeySecretVersion.name;
export const epochDelayAlertPolicyId = epochMonitor.alertPolicy.id;
export const epochDelayNotificationChannelId = epochMonitor.notificationChannel.id;
