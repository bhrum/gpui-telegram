//! Subsystem tests for UI Effects and Modal Layer Stack (`ui/effects` & `ui/layers`).
//! Validates Mandate #18 anti-bluffing exact math, FSM state machines, and GPUI components.

use gpui_telegram_desktop::components::ui::effects::layer_backdrop::*;
use gpui_telegram_desktop::components::ui::effects::peer_qr_layer::*;
use gpui_telegram_desktop::components::ui::effects::premium_coin::{SAMPLE_COUNT, SPIN_DURATION};
use gpui_telegram_desktop::components::ui::effects::premium_star::MAX_PITCH;
use gpui_telegram_desktop::components::ui::effects::round_checkbox::*;
use gpui_telegram_desktop::components::ui::effects::send_action_animations::*;
use gpui_telegram_desktop::components::ui::effects::skeleton_animation::*;
use gpui_telegram_desktop::components::ui::effects::*;
use gpui_telegram_desktop::components::ui::layers::*;
use gpui_telegram_desktop::views::app_shell::ModalContainer;

#[test]
fn test_standalone_layer_stack_push_pop_fsm() {
    let mut stack = StandaloneLayerStackLayers::new();
    assert!(!stack.has_active_layers());
    assert_eq!(stack.layer_count(), 0);

    // Push first dialog
    let id1 = stack.show_box("Delete Message", 400.0, 200.0, LayerStackOption::Normal);
    assert!(stack.has_active_layers());
    assert_eq!(stack.layer_count(), 1);
    assert_eq!(stack.current_box().unwrap().id, id1);
    assert_eq!(stack.current_box().unwrap().title, "Delete Message");
    assert_eq!(stack.box_added_events(), 1);

    // Push second dialog on top (stacked)
    let id2 = stack.show_box("Confirm Action", 360.0, 180.0, LayerStackOption::Normal);
    assert_eq!(stack.layer_count(), 2);
    assert_eq!(stack.current_box().unwrap().id, id2);
    assert_eq!(stack.current_box().unwrap().title, "Confirm Action");

    // Close top dialog -> reveals first dialog
    let popped = stack.close_top();
    assert!(popped.is_some());
    assert_eq!(popped.unwrap().id, id2);
    assert_eq!(stack.layer_count(), 1);
    assert_eq!(stack.current_box().unwrap().id, id1);
    assert_eq!(stack.box_closed_events(), 1);

    // Test CloseOther option
    let _id3 = stack.show_box("Exclusive Dialog", 500.0, 300.0, LayerStackOption::CloseOther);
    assert_eq!(stack.layer_count(), 1);
    assert_eq!(stack.current_box().unwrap().title, "Exclusive Dialog");

    // Hide all
    stack.hide_layers();
    assert_eq!(stack.layer_count(), 0);
    assert!(!stack.has_active_layers());
}

#[test]
fn test_layer_backdrop_blur_and_crossfade_math() {
    let mut backdrop = LayerBackdropEffects::new();
    assert_eq!(BLUR_RADIUS, 24.0);
    assert_eq!(DOWNSCALE_FACTOR, 4.0);

    backdrop.start(true); // night mode
    assert!(backdrop.is_started());
    let dim_night = backdrop.dim_opacity(1.0);
    assert!(dim_night >= 0.6);

    backdrop.start(false); // day mode
    let dim_day = backdrop.dim_opacity(1.0);
    assert!(dim_day < dim_night);

    // Cross-fade formula test
    backdrop.set_progress(0.0);
    let (back, front) = backdrop.cross_fade_weights(1.0);
    assert!((back - 1.0).abs() < 0.01);
    assert!((front - 0.0).abs() < 0.01);

    backdrop.set_progress(1.0);
    let (back, front) = backdrop.cross_fade_weights(1.0);
    assert_eq!(back, 0.0);
    assert_eq!(front, 1.0);
}

#[test]
fn test_round_checkbox_two_phase_animation() {
    let mut cb = RoundCheckboxEffects::new(24.0);
    assert_eq!(WIDE_SCALE, 3);
    assert_eq!(cb.inner_punchout_diameter(), 24.0);

    // Phase 1: progress at 0.25 (half of bg_duration 0.5)
    cb.set_progress(0.25);
    assert!((cb.round_progress() - 0.5).abs() < 0.01);
    assert_eq!(cb.check_progress(), 0.0);
    assert_eq!(cb.check_divider_width(), 0.0);
    assert!((cb.inner_punchout_diameter() - 12.0).abs() < 0.01);

    // Phase 2: progress at 0.75 (half of fg_duration 0.5)
    cb.set_progress(0.75);
    assert_eq!(cb.round_progress(), 1.0);
    assert!((cb.check_progress() - 0.5).abs() < 0.01);
    assert!((cb.check_divider_width() - 12.0).abs() < 0.01);
    assert_eq!(cb.inner_punchout_diameter(), 0.0);

    // Fully checked
    cb.set_progress(1.0);
    assert_eq!(cb.round_progress(), 1.0);
    assert_eq!(cb.check_progress(), 1.0);
    assert_eq!(cb.check_divider_width(), 24.0);
}


#[test]
fn test_skeleton_animation_duty_cycle() {
    let mut sk = SkeletonAnimationEffects::new(16.0);
    assert_eq!(FULL_DURATION.as_millis(), 2000);
    assert_eq!(SLIDE_DURATION.as_millis(), 1000);
    assert_eq!(WAIT_DURATION.as_millis(), 1000);

    sk.start();
    assert!(sk.is_active());

    // In sliding half (e.g. 500ms)
    sk.tick(500);
    assert!(sk.is_gradient_active());
    assert!((sk.slide_progress() - 0.5).abs() < 0.01);

    // In waiting half (e.g. 1500ms total)
    sk.tick(1000);
    assert!(!sk.is_gradient_active());
    assert_eq!(sk.slide_progress(), 1.0);

    let (thick, rad) = sk.line_metrics();
    assert_eq!(thick, 8.0);
    assert_eq!(rad, 4.0);
}

#[test]
fn test_scroll_content_shadow_visibility_math() {
    let mut shadow = ScrollContentShadowEffects::new();
    // At top: no top shadow, yes bottom shadow
    shadow.update_geometry(0.0, 400.0, 1000.0);
    assert!(!shadow.is_top_shadow_visible());
    assert!(shadow.is_bottom_shadow_visible());

    // Scrolled middle: both visible
    shadow.update_geometry(200.0, 400.0, 1000.0);
    assert!(shadow.is_top_shadow_visible());
    assert!(shadow.is_bottom_shadow_visible());

    // At bottom: top visible, no bottom shadow
    shadow.update_geometry(600.0, 400.0, 1000.0);
    assert!(shadow.is_top_shadow_visible());
    assert!(!shadow.is_bottom_shadow_visible());
}

#[test]
fn test_shake_animation_5_segment_oscillation() {
    let mut shake = ShakeAnimationEffects::new(10.0);
    shake.start();
    assert!(shake.is_shaking());

    // Segment 0 start (progress 0.0) -> shift 0.0
    shake.set_progress(0.0);
    assert_eq!(shake.current_pixel_offset(), 0.0);

    // Segment 0 end / Segment 1 start (~progress 0.1667 -> full_progress 1.0) -> positive peak shift
    shake.set_progress(0.1667);
    let s1 = shake.current_pixel_offset();
    assert!(s1 > 8.0);

    // Segment 1 end / Segment 2 start (~progress 0.3333 -> full_progress 2.0) -> negative peak shift
    shake.set_progress(0.3333);
    let s2 = shake.current_pixel_offset();
    assert!(s2 < -8.0);

    // End of shake -> finished
    shake.set_progress(1.0);
    assert!(!shake.is_shaking());
}

#[test]
fn test_send_action_typing_ease_out_circ() {
    let mut action = SendActionAnimationsEffects::new(SendActionType::Typing);
    assert_eq!(action.action_type(), SendActionType::Typing);

    // Check all 3 dots have positive scale
    for i in 0..TYPING_DOTS_COUNT {
        let scale = action.dot_scale(i);
        assert!(scale >= 1.0 && scale <= 1.6, "Dot {} scale {} out of bounds", i, scale);
    }

    action.tick(300);
    for i in 0..TYPING_DOTS_COUNT {
        let scale = action.dot_scale(i);
        assert!(scale >= 1.0 && scale <= 1.6);
    }
}

#[test]
fn test_peer_qr_falloff_and_panel_math() {
    // Test Simpson falloff factor equation
    let f0 = PeerQrLayerEffects::falloff_factor(0.0);
    assert!((f0 - FALLOFF_MIN).abs() < 0.001);

    let f1 = PeerQrLayerEffects::falloff_factor(1.0);
    assert!((f1 - 1.0).abs() < 0.001);

    // Test panel cubic ease-out
    assert_eq!(PeerQrPanelEffects::content_in_progress(50), 0.0);
    let p_mid = PeerQrPanelEffects::content_in_progress(170); // 110ms into 220ms
    assert!(p_mid > 0.5); // Ease out is front-loaded
    assert_eq!(PeerQrPanelEffects::content_in_progress(300), 1.0);
}

#[test]
fn test_premium_3d_pitch_and_spin_parameters() {
    let mut star = PremiumStarEffects::new();
    star.set_rotation(45.0, 75.0); // should clamp to 50.0
    assert_eq!(star.pitch(), MAX_PITCH);

    star.set_rotation(45.0, -80.0); // should clamp to -50.0
    assert_eq!(star.pitch(), -MAX_PITCH);

    assert_eq!(SPIN_DURATION.as_millis(), 3000);
    assert_eq!(SAMPLE_COUNT, 4);
}

#[test]
fn test_modal_container_wiring() {
    let mut container = ModalContainer::new();
    assert!(!container.is_modal_active());

    container.show_modal("Edit Channel", 500.0, 350.0, LayerStackOption::Normal);
    assert!(container.is_modal_active());
    assert_eq!(container.current_box().unwrap().title, "Edit Channel");

    let dismissed = container.dismiss_top();
    assert!(dismissed.is_some());
    assert_eq!(dismissed.unwrap().title, "Edit Channel");
    assert!(!container.is_modal_active());
}
