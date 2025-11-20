import * as grafana from '@pulumiverse/grafana';
import { grafanaProvider } from './providers';

/**
 * Creates an escalation chain for SEV-1:
 * 1. Declare an incident of Critical severity.
 * 2. Start Important notification for the Primary schedule.
 * 3. Start Important notification for the Secondary schedule.
 * 4. Start Important notification for the entire Lit Protocol Developers team.
 */
export function createSev1EscalationChain(
  team: grafana.oncall.GetTeamResult,
  primarySchedule: grafana.oncall.Schedule,
  secondarySchedule: grafana.oncall.Schedule,
  nameSuffix: string
): grafana.oncall.EscalationChain {
  const escalationChain = new grafana.oncall.EscalationChain(
    `${team.name} - Sev1`,
    {
      name: `Sev1 Escalation Chain ${nameSuffix}`,
      teamId: team.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );

  // Create the escalations.
  const declareIncident = new grafana.oncall.Escalation(
    `${team.name} - Sev1 - Declare-Incident`,
    {
      position: 0,
      type: 'declare_incident',
      escalationChainId: escalationChain.id,
      severity: 'critical',
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation1 = new grafana.oncall.Escalation(
    `${team.name} - Sev1 - Escalation-1`,
    {
      position: 1,
      type: 'notify_on_call_from_schedule',
      important: true,
      notifyOnCallFromSchedule: primarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const wait = new grafana.oncall.Escalation(
    `${team.name} - Sev1 - Wait`,
    {
      position: 2,
      type: 'wait',
      duration: 120,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation2 = new grafana.oncall.Escalation(
    `${team.name} - Sev1 - Escalation-2`,
    {
      position: 3,
      type: 'notify_on_call_from_schedule',
      important: true,
      notifyOnCallFromSchedule: secondarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation3 = new grafana.oncall.Escalation(
    `${team.name} - Sev1 - Escalation-3`,
    {
      position: 4,
      type: 'notify_team_members',
      important: true,
      notifyToTeamMembers: team.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );

  return escalationChain;
}

/**
 * Creates an escalation chain for SEV-2:
 * 1. Declare an incident of Critical severity.
 * 2. Start Important notification for the Primary schedule.
 * 3. Start Important notification for the Secondary schedule.
 * 4. Wait 1 minutes
 * 5. Start Important notification for the entire Lit Protocol Developers team.
 * @param team The team to create the escalation chain for.
 * @param primarySchedule The primary schedule to notify.
 * @param secondarySchedule The secondary schedule to notify.
 */
export function createSev2EscalationChain(
  team: grafana.oncall.GetTeamResult,
  primarySchedule: grafana.oncall.Schedule,
  secondarySchedule: grafana.oncall.Schedule,
  adminTeam: grafana.oncall.GetTeamResult,
  nameSuffix: string
): grafana.oncall.EscalationChain {
  const escalationChain = new grafana.oncall.EscalationChain(
    `${team.name} - Sev2`,
    {
      name: `Sev2 Escalation Chain ${nameSuffix}`,
      teamId: team.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );

  // Create the escalations.
  const declareIncident = new grafana.oncall.Escalation(
    `${team.name} - Sev2 - Declare-Incident`,
    {
      position: 0,
      type: 'declare_incident',
      escalationChainId: escalationChain.id,
      severity: 'critical',
    },
    { provider: grafanaProvider }
  );
  const escalation1 = new grafana.oncall.Escalation(
    `${team.name} - Sev2 - Escalation-1`,
    {
      position: 1,
      type: 'notify_on_call_from_schedule',
      important: true,
      notifyOnCallFromSchedule: primarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const notifyAdmins = new grafana.oncall.Escalation(
    `${team.name} - Sev2 - Notify-Admins`,
    {
      position: 2,
      type: 'notify_team_members',
      important: true,
      notifyToTeamMembers: adminTeam.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const wait = new grafana.oncall.Escalation(
    `${team.name} - Sev2 - Wait`,
    {
      position: 3,
      type: 'wait',
      duration: 120,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation2 = new grafana.oncall.Escalation(
    `${team.name} - Sev2 - Escalation-2`,
    {
      position: 4,
      type: 'notify_on_call_from_schedule',
      important: true,
      notifyOnCallFromSchedule: secondarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation3 = new grafana.oncall.Escalation(
    `${team.name} - Sev2 - Escalation-3`,
    {
      position: 5,
      type: 'wait',
      duration: 300,
      important: true,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation4 = new grafana.oncall.Escalation(
    `${team.name} - Sev2 - Escalation-4`,
    {
      position: 6,
      type: 'notify_team_members',
      important: true,
      notifyToTeamMembers: team.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );

  return escalationChain;
}

/**
 * Creates an escalation chain for SEV-3 and below:
 * 1. Declare an incident of Major severity.
 * 2. Start Important notification for the Primary schedule.
 * 3. Wait 1 minutes
 * 4. Start Important notification for the Secondary schedule.
 * 5. Wait 1 minutes
 * 6. Start Important notification for the entire Lit Protocol Developers team.
 * @param team The team to create the escalation chain for.
 * @param primarySchedule The primary schedule to notify.
 * @param secondarySchedule The secondary schedule to notify.
 */
export function createSev3EscalationChain(
  team: grafana.oncall.GetTeamResult,
  primarySchedule: grafana.oncall.Schedule,
  secondarySchedule: grafana.oncall.Schedule,
  nameSuffix: string
): grafana.oncall.EscalationChain {
  const escalationChain = new grafana.oncall.EscalationChain(
    `${team.name} - Sev3`,
    {
      name: `Sev3 Escalation Chain ${nameSuffix}`,
      teamId: team.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'], // Only replace when name changes
    }
  );

  // Create the escalations.
  const declareIncident = new grafana.oncall.Escalation(
    `${team.name} - Sev3 - Declare-Incident`,
    {
      position: 0,
      type: 'declare_incident',
      escalationChainId: escalationChain.id,
      severity: 'major',
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation1 = new grafana.oncall.Escalation(
    `${team.name} - Sev3 - Escalation-1`,
    {
      position: 1,
      type: 'notify_on_call_from_schedule',
      important: true,
      notifyOnCallFromSchedule: primarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation2 = new grafana.oncall.Escalation(
    `${team.name} - Sev3 - Escalation-2`,
    {
      position: 2,
      type: 'wait',
      duration: 300,
      important: true,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation3 = new grafana.oncall.Escalation(
    `${team.name} - Sev3 - Escalation-3`,
    {
      position: 3,
      type: 'notify_on_call_from_schedule',
      important: true,
      notifyOnCallFromSchedule: secondarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation4 = new grafana.oncall.Escalation(
    `${team.name} - Sev3 - Escalation-4`,
    {
      position: 4,
      type: 'wait',
      duration: 300,
      important: true,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation5 = new grafana.oncall.Escalation(
    `${team.name} - Sev3 - Escalation-5`,
    {
      position: 5,
      type: 'notify_team_members',
      important: true,
      notifyToTeamMembers: team.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );

  return escalationChain;
}

/**
 * Creates an escalation chain for SEV-4 and below:
 * 1. Declare an incident of Minor severity.
 * 2. Start Default notification for the Primary schedule.
 * 3. Wait 5 minutes
 * 4. Start Default notification for the Secondary schedule.
 * 5. Wait 5 minutes
 * 5. Start Default notification for the entire Lit Protocol Developers team.
 * @param team The team to create the escalation chain for.
 * @param primarySchedule The primary schedule to notify.
 * @param secondarySchedule The secondary schedule to notify.
 */
export function createSev4OrLessEscalationChain(
  team: grafana.oncall.GetTeamResult,
  primarySchedule: grafana.oncall.Schedule,
  secondarySchedule: grafana.oncall.Schedule,
  nameSuffix: string
): grafana.oncall.EscalationChain {
  const escalationChain = new grafana.oncall.EscalationChain(
    `${team.name} - Sev4OrLess`,
    {
      name: `Sev4OrLess Escalation Chain ${nameSuffix}`,
      teamId: team.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );

  // Create the escalations.
  const declareIncident = new grafana.oncall.Escalation(
    `${team.name} - Sev4OrLess - Declare-Incident`,
    {
      position: 0,
      type: 'declare_incident',
      escalationChainId: escalationChain.id,
      severity: 'minor',
    },
    { provider: grafanaProvider }
  );
  const escalation1 = new grafana.oncall.Escalation(
    `${team.name} - Sev4OrLess - Escalation-1`,
    {
      position: 1,
      type: 'notify_on_call_from_schedule',
      notifyOnCallFromSchedule: primarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation2 = new grafana.oncall.Escalation(
    `${team.name} - Sev4OrLess - Escalation-2`,
    {
      position: 2,
      type: 'wait',
      duration: 600,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation3 = new grafana.oncall.Escalation(
    `${team.name} - Sev4OrLess - Escalation-3`,
    {
      position: 3,
      type: 'notify_on_call_from_schedule',
      notifyOnCallFromSchedule: secondarySchedule.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation4 = new grafana.oncall.Escalation(
    `${team.name} - Sev4OrLess - Escalation-4`,
    {
      position: 4,
      type: 'wait',
      duration: 600,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
  const escalation5 = new grafana.oncall.Escalation(
    `${team.name} - Sev4OrLess - Escalation-5`,
    {
      position: 5,
      type: 'notify_team_members',
      notifyToTeamMembers: team.id,
      escalationChainId: escalationChain.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );

  return escalationChain;
}

/**
 * Creates an empty escalation chain for resolving incidents.
 */
export function createResolveEscalationChain(
  team: grafana.oncall.GetTeamResult,
  nameSuffix: string
): grafana.oncall.EscalationChain {
  return new grafana.oncall.EscalationChain(
    `${team.name} - Resolve`,
    {
      name: `Resolve Escalation Chain ${nameSuffix}`,
      teamId: team.id,
    },
    {
      provider: grafanaProvider,
      deleteBeforeReplace: false,
      replaceOnChanges: ['name'],
    }
  );
}
