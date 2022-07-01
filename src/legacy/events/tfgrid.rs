use crate::events::TFGridEvent;

impl From<pallet_tfgrid_legacy::Event<runtime_legacy::Runtime>> for TFGridEvent {
    fn from(tfge: pallet_tfgrid_legacy::Event<runtime_legacy::Runtime>) -> Self {
        match tfge {
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::FarmStored(farm) => {
                TFGridEvent::FarmStored(farm.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::FarmUpdated(farm) => {
                TFGridEvent::FarmUpdated(farm.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::FarmDeleted(id) => TFGridEvent::FarmDeleted(id),
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::NodeStored(node) => {
                TFGridEvent::NodeStored(node.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::NodeUpdated(node) => {
                TFGridEvent::NodeUpdated(node.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::NodeDeleted(id) => TFGridEvent::NodeDeleted(id),
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::NodeUptimeReported(id, time, uptime) => {
                TFGridEvent::NodeUptimeReported(id, time, uptime)
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::NodePublicConfigStored(id, config) => {
                TFGridEvent::NodePublicConfigStored(id, config.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::EntityStored(entity) => {
                TFGridEvent::EntityStored(entity.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::EntityUpdated(entity) => {
                TFGridEvent::EntityUpdated(entity.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::EntityDeleted(id) => TFGridEvent::EntityDeleted(id),
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::TwinStored(twin) => {
                TFGridEvent::TwinStored(twin.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::TwinUpdated(twin) => {
                TFGridEvent::TwinUpdated(twin.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::TwinEntityStored(twinid, entityid, sig) => {
                TFGridEvent::TwinEntityStored(twinid, entityid, sig)
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::TwinEntityRemoved(twinid, entityid) => {
                TFGridEvent::TwinEntityRemoved(twinid, entityid)
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::TwinDeleted(id) => TFGridEvent::TwinDeleted(id),
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::PricingPolicyStored(policy) => {
                TFGridEvent::PricingPolicyStored(policy.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::CertificationCodeStored(cc) => {
                TFGridEvent::CertificationCodeStored(cc.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::FarmingPolicyStored(fp) => {
                TFGridEvent::FarmingPolicyStored(fp.into())
            }
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::FarmPayoutV2AddressRegistered(
                id,
                address,
            ) => TFGridEvent::FarmPayoutV2AddressRegistered(id, address),
            pallet_tfgrid_legacy::Event::<runtime_legacy::Runtime>::FarmMarkedAsDedicated(farm_id) => {
                TFGridEvent::FarmMarkedAsDedicated(farm_id)
            }
        }
    }
}
