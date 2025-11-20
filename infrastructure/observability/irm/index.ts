import * as grafana from '@pulumiverse/grafana';
import * as pulumi from '@pulumi/pulumi';
import { grafanaProvider } from './providers';
import {
  createSev1EscalationChain,
  createSev2EscalationChain,
  createSev3EscalationChain,
  createSev4OrLessEscalationChain,
  createResolveEscalationChain,
} from './escalations';

const GRAFANA_USERNAMES = ['adarsh8', 'brendon11', 'litchris', 'howard14'];

const GRAFANA_ADMIN_USERNAMES = ['adamcbca', 'abhishek77dc'];

const slackChannelIds = {
  oncall: 'C081M5SQU8L',
  alerts: 'CB84XKFKQ',
};

// Get the Lit Protocol Developers team.
const webhookIntegrations = pulumi.output(
  grafana.oncall
    .getTeam(
      {
        name: 'Lit Protocol Developers',
      },
      { provider: grafanaProvider }
    )
    .then(async (team) => {
      // Get the admin team.
      const adminTeam = await grafana.oncall.getTeam(
        {
          name: 'Lit Protocol Admins',
        },
        { provider: grafanaProvider }
      );

      // Get the infra team.
      const infraTeam = await grafana.oncall.getTeam(
        {
          name: 'Lit Protocol Infra',
        },
        { provider: grafanaProvider }
      );

      // Get all the users.
      const users = await Promise.all(
        GRAFANA_USERNAMES.map((username) =>
          grafana.oncall.getUser({ username }, { provider: grafanaProvider })
        )
      );
      const adminUsers = await Promise.all(
        GRAFANA_ADMIN_USERNAMES.map((username) =>
          grafana.oncall.getUser({ username }, { provider: grafanaProvider })
        )
      );
      const userIds = users.map((user) => user.id);
      const rollingUsers = users.map((user) => [user.id]);
      const rollingAdminUsers = adminUsers.map((user) => [user.id]);

      // Create user notification rules.
      for (const user of [...users, ...adminUsers]) {
        createUserNotificationRules(user);
      }

      // Create the primaryoncall shift.
      const primaryOncallShift = new grafana.oncall.OnCallShift(
        'primaryShift',
        {
          name: 'Primary Shift',
          duration: 604800, // seconds in a week
          frequency: 'weekly',
          level: 1,
          interval: 1,
          weekStart: 'WE',
          rollingUsers,
          teamId: team.id,
          type: 'rolling_users',
          start: '2024-11-19T00:00:00',
        },
        { provider: grafanaProvider }
      );

      // Create the secondary oncall shift.
      const secondaryOncallShift = new grafana.oncall.OnCallShift(
        'secondaryShift',
        {
          name: 'Secondary Shift',
          duration: 604800, // seconds in a week
          frequency: 'weekly',
          level: 1,
          interval: 1,
          weekStart: 'WE',
          rollingUsers,
          startRotationFromUserIndex: 2,
          teamId: team.id,
          type: 'rolling_users',
          start: '2024-11-19T00:00:00',
        },
        { provider: grafanaProvider }
      );

      // Create the primary oncall schedule for the infra team.
      const infraPrimaryOncallShift = new grafana.oncall.OnCallShift(
        'infraPrimaryShift',
        {
          name: 'Infra Team Primary Shift',
          duration: 604800, // seconds in a week
          frequency: 'weekly',
          level: 1,
          interval: 1,
          weekStart: 'WE',
          rollingUsers: rollingAdminUsers,
          teamId: infraTeam.id,
          type: 'rolling_users',
          start: '2024-11-19T00:00:00',
        },
        { provider: grafanaProvider }
      );

      // Create the secondary oncall schedule for the infra team.
      const infraSecondaryOncallShift = new grafana.oncall.OnCallShift(
        'infraSecondaryShift',
        {
          name: 'Infra Team Secondary Shift',
          duration: 604800, // seconds in a week
          frequency: 'weekly',
          level: 1,
          interval: 1,
          weekStart: 'WE',
          rollingUsers: rollingAdminUsers,
          startRotationFromUserIndex: 1,
          teamId: infraTeam.id,
          type: 'rolling_users',
          start: '2024-11-19T00:00:00',
        },
        { provider: grafanaProvider }
      );

      // Create the oncall schedule for Node Primary responders.
      const nodePrimarySchedule = new grafana.oncall.Schedule(
        'Node - Primary',
        {
          name: 'Node - Primary Schedule',
          teamId: team.id,
          shifts: [primaryOncallShift.id],
          slack: {
            channelId: slackChannelIds.oncall,
          },
          type: 'calendar',
          timeZone: 'America/Los_Angeles',
        },
        { provider: grafanaProvider }
      );

      // Create the oncall schedule for Node Secondary responders.
      const nodeSecondarySchedule = new grafana.oncall.Schedule(
        'Node - Secondary',
        {
          name: 'Node - Secondary Schedule',
          teamId: team.id,
          shifts: [secondaryOncallShift.id],
          slack: {
            channelId: slackChannelIds.oncall,
          },
          type: 'calendar',
          timeZone: 'America/Los_Angeles',
        },
        { provider: grafanaProvider }
      );

      // Create the oncall schedule for the infra team primary responders.
      const infraPrimarySchedule = new grafana.oncall.Schedule(
        'Infra - Primary',
        {
          name: 'Infra - Primary Schedule',
          teamId: infraTeam.id,
          shifts: [infraPrimaryOncallShift.id],
          slack: {
            channelId: slackChannelIds.oncall,
          },
          type: 'calendar',
          timeZone: 'America/Los_Angeles',
        },
        { provider: grafanaProvider }
      );

      // Create the oncall schedule for the infra team secondary responders.
      const infraSecondarySchedule = new grafana.oncall.Schedule(
        'Infra - Secondary',
        {
          name: 'Infra - Secondary Schedule',
          teamId: infraTeam.id,
          shifts: [infraSecondaryOncallShift.id],
          slack: {
            channelId: slackChannelIds.oncall,
          },
          type: 'calendar',
          timeZone: 'America/Los_Angeles',
        },
        { provider: grafanaProvider }
      );

      // Create escalation chains for the node team.
      const sev1EscalationChain = createSev1EscalationChain(
        team,
        nodePrimarySchedule,
        nodeSecondarySchedule,
        '- App'
      );
      const sev2EscalationChain = createSev2EscalationChain(
        team,
        nodePrimarySchedule,
        nodeSecondarySchedule,
        adminTeam,
        '- App'
      );
      const sev3EscalationChain = createSev3EscalationChain(
        team,
        nodePrimarySchedule,
        nodeSecondarySchedule,
        '- App'
      );
      const sev4OrLessEscalationChain = createSev4OrLessEscalationChain(
        team,
        nodePrimarySchedule,
        nodeSecondarySchedule,
        '- App'
      );
      const resolveEscalationChain = createResolveEscalationChain(team, '- App');

      // Create escalation chains for the infra team.
      const infraSev1EscalationChain = createSev1EscalationChain(
        infraTeam,
        infraPrimarySchedule,
        infraSecondarySchedule,
        '- Infra'
      );
      const infraSev2EscalationChain = createSev2EscalationChain(
        infraTeam,
        infraPrimarySchedule,
        infraSecondarySchedule,
        adminTeam,
        '- Infra'
      );
      const infraSev3EscalationChain = createSev3EscalationChain(
        infraTeam,
        infraPrimarySchedule,
        infraSecondarySchedule,
        '- Infra'
      );
      const infraSev4OrLessEscalationChain = createSev4OrLessEscalationChain(
        infraTeam,
        infraPrimarySchedule,
        infraSecondarySchedule,
        '- Infra'
      );
      const infraResolveEscalationChain = createResolveEscalationChain(
        infraTeam,
        '- Infra'
      );

      // Create the GCP webhook integration.
      const gcpWebhookIntegration = new grafana.oncall.Integration(
        'gcpWebhookIntegration',
        {
          name: 'GCP Webhook Integration',
          type: 'webhook',
          teamId: team.id,
          templates: {
            slack: {
              title: `{% if payload.incident.state == 'open' %}{% if payload.incident.severity == 'Critical' %}:red_circle: [CRITICAL] FIRING{% elif payload.incident.severity == 'Error' %}:orange_circle: [ERROR] FIRING{% elif payload.incident.severity == 'Warning' %}:yellow_circle: [WARNING] FIRING{% else %}:blue_circle: [INFO] FIRING{% endif %}{% else %}:white_check_mark: [RESOLVED]{% endif %}: {{ payload.incident.condition_name }}`,
              message:
                `📊 *Alert Details*\n` +
                `> *Policy:* {{ payload.incident.policy_name | default("Unknown") }}\n` +
                `> *Project:* \`{{ payload.incident.resource.labels.project_id | default("Unknown") }}\`\n` +
                `> *Started:* <!date^{{ payload.incident.started_at }}^{date_short_pretty} at {time}|{{ payload.incident.started_at | default("Unknown") }}>\n\n` +
                `📈 *Metric Information*\n` +
                `> *Observed Value:* \`{{ payload.incident.observed_value | default("N/A") }}\`\n` +
                `> *Threshold:* \`> {{ payload.incident.threshold_value | default("N/A") }}\`\n` +
                `> *Metric:* \`{{ payload.incident.metric.displayName | default("Unknown") }}\`\n` +
                `{% if payload.incident.metric.labels and payload.incident.metric.labels|length > 0 %}> *Labels:*\n{% for key, value in payload.incident.metric.labels.items() %}>   • \`{{ key }}\`: \`{{ value }}\`\n{% endfor %}{% endif %}\n` +
                `> *Response Playbook:*\n` +
                `> {{ payload.incident.documentation.content | default("No playbook available.") | replace("\n", "\n> ") }}\n\n` +
                `---\n` +
                `:cloud: <{{ payload.incident.url }}|*View Metric in GCP Console*>`,
            },
          },
          defaultRoute: {
            escalationChainId: sev3EscalationChain.id,
            slack: {
              channelId: slackChannelIds.alerts,
            },
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      // Create the Sentry webhook integration.
      const sentryWebhookIntegration = new grafana.oncall.Integration(
        'sentryWebhookIntegration',
        {
          name: 'Sentry Webhook Integration',
          type: 'webhook',
          teamId: team.id,
          defaultRoute: {
            escalationChainId: sev4OrLessEscalationChain.id,
            slack: {
              channelId: slackChannelIds.alerts,
            },
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      // Create the Zendesk webhook integration.
      const zendeskWebhookIntegration = new grafana.oncall.Integration(
        'zendeskWebhookIntegration',
        {
          name: 'Zendesk Webhook Integration',
          type: 'webhook',
          teamId: infraTeam.id,
          defaultRoute: {
            escalationChainId: infraSev3EscalationChain.id,
            slack: {
              channelId: slackChannelIds.alerts,
            },
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      // Route each severity level to the corresponding escalation chain.

      // Auto-resolve incidents when GCP sends a 'closed' state.
      const resolveRoute = new grafana.oncall.Route(
        'resolveRoute',
        {
          integrationId: gcpWebhookIntegration.id,
          position: 0,
          routingRegex: '{{ payload.incident.state == "closed" }}',
          routingType: 'jinja2',
          escalationChainId: resolveEscalationChain.id,
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      const sev2Route = new grafana.oncall.Route(
        'sev2Route',
        {
          integrationId: gcpWebhookIntegration.id,
          position: 1,
          routingRegex: '{{ payload.incident.severity == "critical" }}',
          routingType: 'jinja2',
          escalationChainId: sev2EscalationChain.id,
          slack: {
            channelId: slackChannelIds.alerts,
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      const sev3Route = new grafana.oncall.Route(
        'sev3Route',
        {
          integrationId: gcpWebhookIntegration.id,
          position: 2,
          routingRegex: '{{ payload.incident.severity == "error" }}',
          routingType: 'jinja2',
          escalationChainId: sev3EscalationChain.id,
          slack: {
            channelId: slackChannelIds.alerts,
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      const sev4OrLessRoute = new grafana.oncall.Route(
        'sev4OrLessRoute',
        {
          integrationId: gcpWebhookIntegration.id,
          position: 3,
          routingRegex: '{{ payload.incident.severity <= "warning" }}',
          routingType: 'jinja2',
          escalationChainId: sev4OrLessEscalationChain.id,
          slack: {
            channelId: slackChannelIds.alerts,
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      // Create one escalation route for the Sentry webhook integration.
      // We only have one route for the Sentry webhook integration since Sentry does not distinguish
      // between different severity levels.
      // NOTE: We do NOT have a sev1 route at this moment.
      const sev4RouteSentry = new grafana.oncall.Route(
        'sev4Route-Sentry',
        {
          integrationId: sentryWebhookIntegration.id,
          position: 0,
          routingRegex: '{{ payload.data.event.type == "error" }}',
          routingType: 'regex',
          escalationChainId: sev4OrLessEscalationChain.id,
          slack: {
            channelId: slackChannelIds.alerts,
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      // Create routes for the Zendesk webhook integration.
      const infraSev2Route = new grafana.oncall.Route(
        'infraSev2Route',
        {
          integrationId: zendeskWebhookIntegration.id,
          position: 1,
          routingRegex:
            '{{ "sev1" in payload.ticket.tags or "sev2" in payload.ticket.tags }}',
          routingType: 'jinja2',
          escalationChainId: infraSev2EscalationChain.id,
          slack: {
            channelId: slackChannelIds.alerts,
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      const infraSev3Route = new grafana.oncall.Route(
        'infraSev3Route',
        {
          integrationId: zendeskWebhookIntegration.id,
          position: 2,
          routingRegex: '{{ "sev3" in payload.ticket.tags }}',
          routingType: 'jinja2',
          escalationChainId: infraSev3EscalationChain.id,
          slack: {
            channelId: slackChannelIds.alerts,
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      const infraSev4OrLessRoute = new grafana.oncall.Route(
        'infraSev4OrLessRoute',
        {
          integrationId: zendeskWebhookIntegration.id,
          position: 3,
          routingRegex: '{{ "sev4" in payload.ticket.tags }}',
          routingType: 'jinja2',
          escalationChainId: infraSev4OrLessEscalationChain.id,
          slack: {
            channelId: slackChannelIds.alerts,
          },
        },
        {
          provider: grafanaProvider,
          deleteBeforeReplace: false,
          replaceOnChanges: ['name'],
        }
      );

      return {
        gcpIntegration: {
          integration: gcpWebhookIntegration,
          id: gcpWebhookIntegration.id,
          webhookUrl: gcpWebhookIntegration.link,
          token: gcpWebhookIntegration.id.apply(
            (id) => `Basic ${Buffer.from(id).toString('base64')}`
          ),
        },
        sentryIntegration: {
          integration: sentryWebhookIntegration,
          id: sentryWebhookIntegration.id,
          webhookUrl: sentryWebhookIntegration.link,
          token: sentryWebhookIntegration.id.apply(
            (id) => `Basic ${Buffer.from(id).toString('base64')}`
          ),
        },
        zendeskIntegration: {
          integration: zendeskWebhookIntegration,
          id: zendeskWebhookIntegration.id,
          webhookUrl: zendeskWebhookIntegration.link,
          token: zendeskWebhookIntegration.id.apply(
            (id) => `Basic ${Buffer.from(id).toString('base64')}`
          ),
        },
      };
    })
);

export const gcpWebhookIntegrationId = webhookIntegrations.apply(
  (x) => x.gcpIntegration.id
);
export const gcpWebhookIntegrationUrl = webhookIntegrations.apply(
  (x) => x.gcpIntegration.webhookUrl
);
export const gcpWebhookIntegrationToken = webhookIntegrations.apply(
  (x) => x.gcpIntegration.token
);
export const sentryWebhookIntegrationId = webhookIntegrations.apply(
  (x) => x.sentryIntegration.id
);
export const sentryWebhookIntegrationUrl = webhookIntegrations.apply(
  (x) => x.sentryIntegration.webhookUrl
);
export const sentryWebhookIntegrationToken = webhookIntegrations.apply(
  (x) => x.sentryIntegration.token
);
export const zendeskWebhookIntegrationId = webhookIntegrations.apply(
  (x) => x.zendeskIntegration.id
);
export const zendeskWebhookIntegrationUrl = webhookIntegrations.apply(
  (x) => x.zendeskIntegration.webhookUrl
);
export const zendeskWebhookIntegrationToken = webhookIntegrations.apply(
  (x) => x.zendeskIntegration.token
);

/**
 * Creates the following rules:
 * - Default:
 *   - Notify by email
 *   - Notify by slack mentions
 *   - Wait 5 minutes
 *   - Notify by SMS
 *   - Notify by mobile push
 * - Important:
 *   - Notify by email
 *   - Notify by slack mentions
 *   - Notify by SMS
 *   - Wait 1 minute
 *   - Notify by phone call
 *   - Notify by mobile push important
 * @param user The user to create the rules for.
 */
function createUserNotificationRules(user: grafana.oncall.GetUserResult) {
  // Create the default rules.
  new grafana.oncall.UserNotificationRule(
    `${user.username}-Default-Email`,
    {
      userId: user.id,
      type: 'notify_by_email',
      position: 0,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Default-Slack`,
    {
      userId: user.id,
      type: 'notify_by_slack',
      position: 1,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Default-Wait`,
    {
      userId: user.id,
      type: 'wait',
      duration: 300,
      position: 2,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Default-SMS`,
    {
      userId: user.id,
      type: 'notify_by_sms',
      position: 3,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Default-MobilePush`,
    {
      userId: user.id,
      type: 'notify_by_mobile_app',
      position: 4,
    },
    { provider: grafanaProvider }
  );

  // Create the important rules.
  new grafana.oncall.UserNotificationRule(
    `${user.username}-Important-Email`,
    {
      userId: user.id,
      type: 'notify_by_email',
      important: true,
      position: 0,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Important-Slack`,
    {
      userId: user.id,
      type: 'notify_by_slack',
      important: true,
      position: 1,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Important-SMS`,
    {
      userId: user.id,
      type: 'notify_by_sms',
      important: true,
      position: 2,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Important-Wait`,
    {
      userId: user.id,
      type: 'wait',
      duration: 60,
      important: true,
      position: 3,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Important-PhoneCall`,
    {
      userId: user.id,
      type: 'notify_by_phone_call',
      important: true,
      position: 4,
    },
    { provider: grafanaProvider }
  );

  new grafana.oncall.UserNotificationRule(
    `${user.username}-Important-MobilePush`,
    {
      userId: user.id,
      type: 'notify_by_mobile_app_critical',
      important: true,
      position: 5,
    },
    { provider: grafanaProvider }
  );
}
