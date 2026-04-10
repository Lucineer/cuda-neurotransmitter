/*!
# cuda-neurotransmitter

Signal-to-gene activation pathways inspired by neurotransmitter systems.

Biological neurons use neurotransmitters (dopamine, serotonin, acetylcholine, etc.)
as chemical signals that bind to receptors and trigger cascading responses.

This crate maps that pattern to agent communication:
- Neurotransmitters = message types (reward, alert, calm, explore, bond)
- Receptors = signal pattern matchers on genes/instincts
- Synapses = connections between agents with strength/decay
- Cascades = one signal triggers multiple downstream activations

The key insight: dopamine IS the confidence signal. Serotonin IS the trust signal.
These aren't metaphors — they're the same mathematical structure (exponential
decay, accumulation, threshold gating).
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Neurotransmitter types — each has a distinct effect on agent behavior
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeuroType {
    /// Reward/prediction error — drives learning, confidence boost
    Dopamine,
    /// Calm/contentment — trust building, social bonding, risk reduction
    Serotonin,
    /// Alert/stress — increases focus, reduces exploration, triggers defense
    Norepinephrine,
    /// Learning/plasticity — temporarily increases gene expression rate
    Acetylcholine,
    /// Pleasure/pain — reinforcement signal for gene fitness
    Endorphin,
    /// Bonding/attachment — increases trust in specific agents
    Oxytocin,
    /// Sleep/repair — triggers rest instinct, memory consolidation
    Melatonin,
    /// Novelty/surprise — resets expectations, drives exploration
    Anandamide,
}

impl NeuroType {
    pub fn all() -> &'static [NeuroType] {
        &[NeuroType::Dopamine, NeuroType::Serotonin, NeuroType::Norepinephrine,
          NeuroType::Acetylcholine, NeuroType::Endorphin, NeuroType::Oxytocin,
          NeuroType::Melatonin, NeuroType::Anandamide]
    }

    pub fn id(self) -> u8 {
        match self {
            NeuroType::Dopamine => 0, NeuroType::Serotonin => 1,
            NeuroType::Norepinephrine => 2, NeuroType::Acetylcholine => 3,
            NeuroType::Endorphin => 4, NeuroType::Oxytocin => 5,
            NeuroType::Melatonin => 6, NeuroType::Anandamide => 7,
        }
    }

    /// How quickly this neurotransmitter decays (half-life in ticks)
    pub fn half_life(self) -> f64 {
        match self {
            NeuroType::Dopamine => 20.0,      // fast decay — sharp reward signal
            NeuroType::Serotonin => 50.0,      // slow decay — sustained trust
            NeuroType::Norepinephrine => 10.0, // very fast — immediate alert
            NeuroType::Acetylcholine => 15.0,  // moderate — temporary plasticity
            NeuroType::Endorphin => 8.0,       // quick hit — reinforcement
            NeuroType::Oxytocin => 40.0,       // slow — lasting bonds
            NeuroType::Melatonin => 100.0,     // very slow — sustained rest
            NeuroType::Anandamide => 12.0,     // fast — fleeting novelty
        }
    }

    /// Base effect magnitude [0,1]
    pub fn base_magnitude(self) -> f64 {
        match self {
            NeuroType::Dopamine => 0.8,
            NeuroType::Serotonin => 0.5,
            NeuroType::Norepinephrine => 0.9,
            NeuroType::Acetylcholine => 0.4,
            NeuroType::Endorphin => 0.7,
            NeuroType::Oxytocin => 0.6,
            NeuroType::Melatonin => 0.3,
            NeuroType::Anandamide => 0.75,
        }
    }
}

/// A neurotransmitter signal with intensity and source
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroSignal {
    pub neuro_type: NeuroType,
    pub intensity: f64,       // 0.0-1.0, how strong
    pub source_agent: String, // who sent it
    pub target_gene: Option<String>, // optional targeted gene
    pub confidence: f64,
    pub age: u32,             // ticks since emission
}

impl NeuroSignal {
    pub fn new(ntype: NeuroType, intensity: f64, source: &str) -> Self {
        NeuroSignal { neuro_type: ntype, intensity: intensity.clamp(0.0, 1.0), source_agent: source.to_string(), target_gene: None, confidence: intensity, age: 0 }
    }

    /// Decay signal based on half-life
    pub fn decay(&mut self) {
        self.age += 1;
        let half_life = self.neuro_type.half_life();
        self.intensity *= (0.5_f64).powf(1.0 / half_life);
        self.confidence *= (0.5_f64).powf(1.0 / half_life);
    }

    /// Is this signal still active?
    pub fn is_active(&self) -> bool { self.intensity > 0.01 }
}

/// A receptor — binds specific neurotransmitter types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receptor {
    pub name: String,
    pub neuro_type: NeuroType,
    /// Sensitivity — how easily this receptor activates [0,1]
    pub sensitivity: f64,
    /// Threshold — signal must exceed this to activate
    pub threshold: f64,
    /// Down-regulation — repeated activation desensitizes
    pub activation_count: u32,
    /// Current desensitization factor [0,1] — 1.0 = fully sensitive
    pub sensitivity_factor: f64,
    /// Which genes this receptor triggers
    pub target_genes: Vec<String>,
}

impl Receptor {
    pub fn new(name: &str, ntype: NeuroType) -> Self {
        Receptor { name: name.to_string(), neuro_type: ntype, sensitivity: 0.5, threshold: 0.1, activation_count: 0, sensitivity_factor: 1.0, target_genes: vec![] }
    }

    /// Try to bind a signal. Returns activation strength [0,1] if activated.
    pub fn bind(&mut self, signal: &NeuroSignal) -> Option<f64> {
        if signal.neuro_type != self.neuro_type { return None; }
        let effective_threshold = self.threshold / self.sensitivity_factor;
        if signal.intensity < effective_threshold { return None; }

        // Down-regulation: more activations = less sensitive
        self.activation_count += 1;
        if self.activation_count > 5 {
            self.sensitivity_factor = 1.0 / (1.0 + (self.activation_count as f64 - 5.0) * 0.05);
            self.sensitivity_factor = self.sensitivity_factor.max(0.2); // floor at 20%
        }

        let strength = (signal.intensity * self.sensitivity_factor * self.sensitivity).clamp(0.0, 1.0);
        Some(strength)
    }

    /// Recovery — sensitivity slowly returns between activations
    pub fn recover(&mut self) {
        self.sensitivity_factor += (1.0 - self.sensitivity_factor) * 0.02;
    }
}

/// A synapse — connection between agents
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Synapse {
    pub pre_agent: String,
    pub post_agent: String,
    /// Connection strength [0,1]
    pub strength: f64,
    /// Signal multiplier
    pub efficacy: f64,
    /// Last signal time (tick number)
    pub last_signal: u32,
    /// Total signals sent
    pub total_signals: u32,
    /// Total signals received
    pub total_received: u32,
}

impl Synapse {
    pub fn new(pre: &str, post: &str) -> Self {
        Synapse { pre_agent: pre.to_string(), post_agent: post.to_string(), strength: 0.5, efficacy: 1.0, last_signal: 0, total_signals: 0, total_received: 0 }
    }

    /// Hebbian learning: "neurons that fire together wire together"
    pub fn hebbian_update(&mut self, pre_fired: bool, post_fired: bool) {
        if pre_fired && post_fired {
            self.strength = (self.strength + 0.01).min(1.0);
        } else if pre_fired && !post_fired {
            self.strength = (self.strength - 0.005).max(0.1);
        }
    }

    /// Process signal through synapse
    pub fn transmit(&mut self, signal: &NeuroSignal) -> NeuroSignal {
        self.total_signals += 1;
        self.last_signal = signal.age;
        let mut out = signal.clone();
        out.intensity *= self.strength * self.efficacy;
        out.confidence *= self.strength;
        out.source_agent = self.pre_agent.clone();
        out
    }

    /// Long-term potentiation (strengthen from repeated use)
    pub fn potentiate(&mut self, amount: f64) {
        self.strength = (self.strength + amount).min(1.0);
        self.efficacy = (self.efficacy + amount * 0.5).min(2.0);
    }

    /// Long-term depression (weaken from disuse)
    pub fn depress(&mut self, amount: f64) {
        self.strength = (self.strength - amount).max(0.1);
        self.efficacy = (self.efficacy - amount * 0.5).max(0.2);
    }
}

/// Synaptic cleft — the signal distribution network
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SynapticCleft {
    pub synapses: HashMap<String, Synapse>, // key = "pre->post"
    pub pending_signals: Vec<NeuroSignal>,
    pub active_signals: Vec<NeuroSignal>,
}

impl SynapticCleft {
    pub fn new() -> Self { SynapticCleft { synapses: HashMap::new(), pending_signals: vec![], active_signals: vec![] } }

    pub fn add_synapse(&mut self, syn: Synapse) {
        let key = format!("{}->{}", syn.pre_agent, syn.post_agent);
        self.synapses.insert(key, syn);
    }

    /// Emit a signal from an agent into the cleft
    pub fn emit(&mut self, signal: NeuroSignal) {
        self.pending_signals.push(signal);
    }

    /// Process all pending signals through synapses
    pub fn flush(&mut self) {
        let pending = std::mem::take(&mut self.pending_signals);
        for signal in pending {
            self.active_signals.push(signal.clone());
            // Transmit through matching synapses
            for syn in self.synapses.values_mut() {
                if syn.pre_agent == signal.source_agent {
                    let transmitted = syn.transmit(&signal);
                    self.active_signals.push(transmitted);
                }
            }
        }
        // Decay all active signals
        self.active_signals.retain_mut(|s| { s.decay(); s.is_active() });
    }

    /// Get all signals targeting a specific agent
    pub fn signals_for(&self, agent: &str) -> Vec<&NeuroSignal> {
        self.active_signals.iter().filter(|s| s.source_agent != agent).collect()
    }

    /// Get dopamine signals specifically (reward/confidence)
    pub fn dopamine_signals(&self) -> Vec<&NeuroSignal> {
        self.active_signals.iter().filter(|s| s.neuro_type == NeuroType::Dopamine).collect()
    }

    /// Net effect on agent state
    pub fn net_effect(&self) -> NeuroEffect {
        let mut eff = NeuroEffect::default();
        for s in &self.active_signals {
            match s.neuro_type {
                NeuroType::Dopamine => eff.confidence_delta += s.intensity * 0.1,
                NeuroType::Serotonin => eff.trust_delta += s.intensity * 0.05,
                NeuroType::Norepinephrine => eff.alertness += s.intensity * 0.15,
                NeuroType::Acetylcholine => eff.learning_rate_delta += s.intensity * 0.1,
                NeuroType::Endorphin => eff.reinforcement += s.intensity * 0.2,
                NeuroType::Oxytocin => eff.bonding_delta += s.intensity * 0.08,
                NeuroType::Melatonin => eff.rest_drive += s.intensity * 0.1,
                NeuroType::Anandamide => eff.exploration_drive += s.intensity * 0.12,
            }
        }
        eff
    }
}

/// Net effect of neurotransmitters on agent state
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NeuroEffect {
    pub confidence_delta: f64,
    pub trust_delta: f64,
    pub alertness: f64,
    pub learning_rate_delta: f64,
    pub reinforcement: f64,
    pub bonding_delta: f64,
    pub rest_drive: f64,
    pub exploration_drive: f64,
}

/// Cascade — one signal triggers multiple downstream activations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cascade {
    pub trigger: NeuroType,
    pub steps: Vec<CascadeStep>,
    pub delay_between_steps: u32,
    pub current_step: usize,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CascadeStep {
    pub neuro_type: NeuroType,
    pub target_genes: Vec<String>,
    pub intensity_multiplier: f64,
}

impl Cascade {
    pub fn new(trigger: NeuroType) -> Self { Cascade { trigger, steps: vec![], delay_between_steps: 1, current_step: 0, active: false } }

    pub fn add_step(&mut self, ntype: NeuroType, genes: Vec<&str>, mult: f64) {
        self.steps.push(CascadeStep { neuro_type: ntype, target_genes: genes.iter().map(|s| s.to_string()).collect(), intensity_multiplier: mult });
    }

    /// Start cascade from a trigger signal
    pub fn start(&mut self, trigger_intensity: f64) -> Option<NeuroSignal> {
        if self.steps.is_empty() { return None; }
        self.active = true;
        self.current_step = 0;
        self.next_step(trigger_intensity)
    }

    /// Get next step signal
    pub fn next_step(&mut self, base_intensity: f64) -> Option<NeuroSignal> {
        if self.current_step >= self.steps.len() { self.active = false; return None; }
        let step = &self.steps[self.current_step];
        let mut sig = NeuroSignal::new(step.neuro_type, base_intensity * step.intensity_multiplier, "cascade");
        sig.target_gene = step.target_genes.first().cloned();
        self.current_step += 1;
        Some(sig)
    }
}

/// Default cascades for common agent behaviors
pub fn default_cascades() -> Vec<Cascade> {
    let mut reward = Cascade::new(NeuroType::Dopamine);
    reward.add_step(NeuroType::Dopamine, vec!["learn", "navigate"], 1.0);
    reward.add_step(NeuroType::Endorphin, vec!["play"], 0.5);
    reward.add_step(NeuroType::Serotonin, vec!["socialize"], 0.3);

    let mut stress = Cascade::new(NeuroType::Norepinephrine);
    stress.add_step(NeuroType::Norepinephrine, vec!["defend", "survive"], 1.0);
    stress.add_step(NeuroType::Melatonin, vec!["rest"], 0.4);

    let mut explore = Cascade::new(NeuroType::Anandamide);
    explore.add_step(NeuroType::Anandamide, vec!["play", "create"], 1.0);
    explore.add_step(NeuroType::Dopamine, vec!["learn"], 0.3);

    vec![reward, stress, explore]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_decay() {
        let mut sig = NeuroSignal::new(NeuroType::Dopamine, 1.0, "agent-1");
        let initial = sig.intensity;
        sig.decay();
        assert!(sig.intensity < initial);
        assert!(sig.intensity > 0.0);
    }

    #[test]
    fn test_receptor_binding() {
        let mut r = Receptor::new("dopamine_r1", NeuroType::Dopamine);
        r.sensitivity = 0.8;
        let sig = NeuroSignal::new(NeuroType::Dopamine, 0.5, "agent-1");
        let result = r.bind(&sig);
        assert!(result.is_some());
        assert!(result.unwrap() > 0.0);
    }

    #[test]
    fn test_receptor_wrong_type() {
        let mut r = Receptor::new("dopamine_r1", NeuroType::Dopamine);
        let sig = NeuroSignal::new(NeuroType::Serotonin, 0.5, "agent-1");
        assert!(r.bind(&sig).is_none());
    }

    #[test]
    fn test_receptor_threshold() {
        let mut r = Receptor::new("r", NeuroType::Dopamine);
        r.threshold = 0.8;
        let sig = NeuroSignal::new(NeuroType::Dopamine, 0.5, "a");
        assert!(r.bind(&sig).is_none()); // below threshold
    }

    #[test]
    fn test_down_regulation() {
        let mut r = Receptor::new("r", NeuroType::Dopamine);
        let sig = NeuroSignal::new(NeuroType::Dopamine, 0.9, "a");
        let initial = r.bind(&sig).unwrap();
        for _ in 0..20 { r.bind(&sig); }
        let later = r.bind(&sig).unwrap();
        assert!(later < initial); // desensitized
    }

    #[test]
    fn test_receptor_recovery() {
        let mut r = Receptor::new("r", NeuroType::Dopamine);
        r.sensitivity_factor = 0.3;
        r.recover();
        assert!(r.sensitivity_factor > 0.3);
    }

    #[test]
    fn test_synapse_transmit() {
        let mut syn = Synapse::new("a", "b");
        syn.strength = 0.8;
        let sig = NeuroSignal::new(NeuroType::Dopamine, 1.0, "a");
        let out = syn.transmit(&sig);
        assert!(out.intensity < 1.0); // reduced by strength
        assert!(out.intensity > 0.0);
    }

    #[test]
    fn test_hebbian_learning() {
        let mut syn = Synapse::new("a", "b");
        let initial = syn.strength;
        syn.hebbian_update(true, true); // both fired
        assert!(syn.strength > initial);
        syn.hebbian_update(true, false); // only pre
        assert!(syn.strength < 1.0); // weakened
    }

    #[test]
    fn test_synaptic_cleft() {
        let mut cleft = SynapticCleft::new();
        cleft.add_synapse(Synapse::new("a", "b"));
        let sig = NeuroSignal::new(NeuroType::Dopamine, 0.8, "a");
        cleft.emit(sig);
        cleft.flush();
        assert!(!cleft.active_signals.is_empty());
    }

    #[test]
    fn test_net_effect() {
        let mut cleft = SynapticCleft::new();
        cleft.emit(NeuroSignal::new(NeuroType::Dopamine, 1.0, "a"));
        cleft.emit(NeuroSignal::new(NeuroType::Serotonin, 1.0, "a"));
        cleft.flush();
        let eff = cleft.net_effect();
        assert!(eff.confidence_delta > 0.0);
        assert!(eff.trust_delta > 0.0);
    }

    #[test]
    fn test_cascade() {
        let mut cascade = Cascade::new(NeuroType::Dopamine);
        cascade.add_step(NeuroType::Dopamine, vec!["learn"], 1.0);
        cascade.add_step(NeuroType::Endorphin, vec!["play"], 0.5);
        let s1 = cascade.start(1.0).unwrap();
        assert!(s1.target_gene == Some("learn".to_string()));
        let s2 = cascade.next_step(1.0).unwrap();
        assert!(s2.target_gene == Some("play".to_string()));
        assert!(cascade.next_step(1.0).is_none()); // done
    }

    #[test]
    fn test_default_cascades() {
        let cascades = default_cascades();
        assert_eq!(cascades.len(), 3);
    }

    #[test]
    fn test_half_lives() {
        // Dopamine should decay faster than serotonin
        assert!(NeuroType::Dopamine.half_life() < NeuroType::Serotonin.half_life());
        // Norepinephrine should decay fastest
        assert!(NeuroType::Norepinephrine.half_life() < NeuroType::Dopamine.half_life());
    }

    #[test]
    fn test_potentiation_depression() {
        let mut syn = Synapse::new("a", "b");
        syn.potentiate(0.1);
        assert!(syn.strength > 0.5);
        syn.depress(0.05);
        assert!(syn.efficacy < 1.5); // reduced from potentiate
    }

    #[test]
    fn test_signals_for_agent() {
        let mut cleft = SynapticCleft::new();
        cleft.emit(NeuroSignal::new(NeuroType::Dopamine, 0.8, "a"));
        cleft.emit(NeuroSignal::new(NeuroType::Serotonin, 0.6, "b"));
        cleft.flush();
        let for_b = cleft.signals_for("b");
        assert!(for_b.iter().any(|s| s.source_agent == "a"));
        assert!(!for_b.iter().any(|s| s.source_agent == "b")); // own signals excluded
    }
}
