use std::sync::Arc;

use crate::error::{Result, parser_err};
use lit_node_core::{
    AccessControlConditionResource, LitActionResource, LitResource, LitResourcePrefix,
    PKPNFTResource, PaymentDelegationResource, ResourceType,
};

pub(crate) fn get_resource_prefix_id_from_type(
    resource_type: &ResourceType,
) -> Result<(String, String)> {
    match resource_type {
        // TODO: code smell - refactor this
        ResourceType::AccessControlCondition(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
        ResourceType::PKPNFT(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
        ResourceType::LitAction(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
        ResourceType::PaymentDelegation(resource) => Ok((
            resource.get_resource_prefix().to_string(),
            resource.get_resource_id().to_owned(),
        )),
    }
}

pub(crate) fn parse_resource_and_prefix<T>(
    resource_id: T,
    resource_prefix: T,
) -> Result<Arc<dyn LitResource>>
where
    T: AsRef<str>,
{
    let resource_prefix = resource_prefix.as_ref();
    let resource_id = resource_id.as_ref();

    match resource_prefix
        .parse::<LitResourcePrefix>()
        .map_err(|e| parser_err("Unable to parse lit resource prefix", None))?
    {
        LitResourcePrefix::ACC => Ok(Arc::new(AccessControlConditionResource::new(
            resource_id.to_owned(),
        ))),
        LitResourcePrefix::PKP => Ok(Arc::new(PKPNFTResource::new(resource_id.to_owned()))),
        LitResourcePrefix::LA => Ok(Arc::new(LitActionResource::new(resource_id.to_owned()))),
        LitResourcePrefix::PD => Ok(Arc::new(PaymentDelegationResource::new(
            resource_id.to_owned(),
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::resources::get_resource_prefix_id_from_type;

    use lit_node_core::{LitAbility, LitResource};

    struct TestCase {
        resource_type: ResourceType,
        expected_prefix: LitResourcePrefix,
        expected_id: String,
    }

    fn get_resource_prefix_id_from_type_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                resource_type: ResourceType::AccessControlCondition(
                    crate::auth::resources::AccessControlConditionResource::new("123".to_string()),
                ),
                expected_prefix: LitResourcePrefix::ACC,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_type: ResourceType::PKPNFT(crate::auth::resources::PKPNFTResource::new(
                    "123".to_string(),
                )),
                expected_prefix: LitResourcePrefix::PKP,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_type: ResourceType::LitAction(
                    crate::auth::resources::LitActionResource::new("123".to_string()),
                ),
                expected_prefix: LitResourcePrefix::LA,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_type: ResourceType::PaymentDelegation(
                    crate::auth::resources::PaymentDelegationResource::new("123".to_string()),
                ),
                expected_prefix: LitResourcePrefix::PD,
                expected_id: "123".to_string(),
            },
        ]
    }

    #[test]
    fn test_get_resource_prefix_id_from_type() {
        let test_cases = get_resource_prefix_id_from_type_test_cases();

        for test_case in test_cases {
            let (prefix, id) = get_resource_prefix_id_from_type(&test_case.resource_type).unwrap();
            assert_eq!(prefix, test_case.expected_prefix.to_string());
            assert_eq!(id, test_case.expected_id);
        }
    }

    #[test]
    fn test_resource_methods() {
        let acc = crate::auth::resources::AccessControlConditionResource::new("123".to_string());
        let pkp = crate::auth::resources::PKPNFTResource::new("123".to_string());
        let la = crate::auth::resources::LitActionResource::new("123".to_string());
        let pd = crate::auth::resources::PaymentDelegationResource::new("123".to_string());

        assert_eq!(acc.get_resource_id(), "123");
        assert_eq!(pkp.get_resource_id(), "123");
        assert_eq!(la.get_resource_id(), "123");
        assert_eq!(pd.get_resource_id(), "123");

        assert_eq!(acc.get_resource_prefix(), LitResourcePrefix::ACC);
        assert_eq!(pkp.get_resource_prefix(), LitResourcePrefix::PKP);
        assert_eq!(la.get_resource_prefix(), LitResourcePrefix::LA);
        assert_eq!(pd.get_resource_prefix(), LitResourcePrefix::PD);

        assert_eq!(
            acc.decrypt_ability().get_ability().to_owned(),
            LitAbility::AccessControlConditionDecryption
        );
        assert_eq!(
            acc.signing_ability().get_ability().to_owned(),
            LitAbility::AccessControlConditionSigning
        );

        assert_eq!(
            pkp.signing_ability().get_ability().to_owned(),
            LitAbility::PKPSigning
        );

        assert_eq!(
            la.execution_ability().get_ability().to_owned(),
            LitAbility::LitActionExecution
        );

        assert_eq!(
            pd.signing_ability().get_ability().to_owned(),
            LitAbility::PaymentDelegationAuth
        );
    }
}

#[cfg(test)]
mod parse_resource_and_prefix_tests {
    use crate::auth::resources::parse_resource_and_prefix;
    use lit_node_core::LitResourcePrefix;

    struct TestCase {
        resource_id: String,
        resource_prefix: String,
        expected_prefix: LitResourcePrefix,
        expected_id: String,
    }

    fn parse_resource_and_prefix_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-accesscontrolcondition".to_string(),
                expected_prefix: LitResourcePrefix::ACC,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-pkp".to_string(),
                expected_prefix: LitResourcePrefix::PKP,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-litaction".to_string(),
                expected_prefix: LitResourcePrefix::LA,
                expected_id: "123".to_string(),
            },
            TestCase {
                resource_id: "123".to_string(),
                resource_prefix: "lit-paymentdelegation".to_string(),
                expected_prefix: LitResourcePrefix::PD,
                expected_id: "123".to_string(),
            },
        ]
    }

    #[test]
    fn test_parse_resource_and_prefix() {
        let test_cases = parse_resource_and_prefix_test_cases();

        for test_case in test_cases {
            let lit_resource =
                parse_resource_and_prefix(&test_case.resource_id, &test_case.resource_prefix)
                    .unwrap();
            assert_eq!(
                lit_resource.get_resource_prefix(),
                test_case.expected_prefix
            );
            assert_eq!(
                lit_resource.get_resource_id().to_owned(),
                test_case.expected_id
            );
        }
    }
}
