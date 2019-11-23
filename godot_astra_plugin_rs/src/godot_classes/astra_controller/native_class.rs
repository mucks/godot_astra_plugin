use super::AstraController;
use gdnative::init::{Property, PropertyHint, PropertyUsage};
use gdnative::*;

impl NativeClass for AstraController {
    type Base = Node;
    type UserData = user_data::MutexData<AstraController>;

    fn class_name() -> &'static str {
        "AstraController"
    }

    fn init(_owner: Self::Base) -> Self {
        unsafe { Self::_init(_owner) }
    }

    fn register_properties(builder: &init::ClassBuilder<Self>) {
        builder.add_property(Property {
            name: "color_enabled",
            default: false,
            hint: PropertyHint::None,
            getter: |this: &AstraController| this.color_enabled,
            setter: |this: &mut AstraController, v| this.color_enabled = v,
            usage: PropertyUsage::DEFAULT,
        });
        builder.add_property(Property {
            name: "masked_color_enabled",
            default: false,
            hint: PropertyHint::None,
            getter: |this: &AstraController| this.masked_color_enabled,
            setter: |this: &mut AstraController, v| this.masked_color_enabled = v,
            usage: PropertyUsage::DEFAULT,
        });
        builder.add_property(Property {
            name: "depth_enabled",
            default: false,
            hint: PropertyHint::None,
            getter: |this: &AstraController| this.depth_enabled,
            setter: |this: &mut AstraController, v| this.depth_enabled = v,
            usage: PropertyUsage::DEFAULT,
        });
        builder.add_property(Property {
            name: "body_enabled",
            default: false,
            hint: PropertyHint::None,
            getter: |this: &AstraController| this.body_enabled,
            setter: |this: &mut AstraController, v| this.body_enabled = v,
            usage: PropertyUsage::DEFAULT,
        });
    }
}
