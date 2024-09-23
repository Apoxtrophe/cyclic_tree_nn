use std::collections::HashMap;

use crate::{convert_f32_to_id, get_inorder_position, get_neuron_height, GeneType, Genome, SynapseStatus};
use plotly::{common::{MarkerSymbol, Mode}, Plot, Scatter};

// Debug display for genome
impl Genome {
    pub fn visualize(&self) {
        let mut plot = Plot::new();
        let mut neuron_positions: HashMap<[u8; 2], (f64, f64)> = HashMap::new();

        // Constants for layout
        let horizontal_spacing = 1.0;
        let vertical_spacing = 1.0;

        // Collect neurons and compute positions
        for gene in &self.genes {
            // Skip synapses
            if GeneType::from_u8(gene.flag[0]) == Some(GeneType::Synapse) {
                continue;
            }

            let neuron_type = GeneType::from_u8(gene.flag[0]).unwrap();
            let mut x_pos = 0.0;
            let mut y_pos = 0.0;

            let tree_id = gene.id[0] as u16;
            let node_id1 = gene.id[1] as u16;

            match neuron_type {
                GeneType::Input => {
                    x_pos = tree_id as f64 * horizontal_spacing;
                    y_pos = 0.0;
                }
                GeneType::Hidden => {
                    let depth = get_neuron_height(node_id1 as u8) as f64;
                    y_pos = depth * vertical_spacing;

                    let inorder_pos = get_inorder_position(node_id1) as f64;
                    let max_nodes_at_depth = 2u32.pow(depth as u32) as f64;

                    x_pos = (inorder_pos + 0.5) * (horizontal_spacing / max_nodes_at_depth);
                    x_pos += tree_id as f64 * horizontal_spacing;
                }
                GeneType::Output => {
                    x_pos = 255.0 - tree_id as f64;
                    y_pos = (7.0 + 1.0) * vertical_spacing;
                }
                _ => {}
            }

            // Store positions
            neuron_positions.insert(gene.id, (x_pos, y_pos));

            // Debug print
            println!("Neuron ID: {:?}, x: {}, y: {}", gene.id, x_pos, y_pos);
        }

        // Collect synapses
        let mut edge_x_enabled = Vec::new();
        let mut edge_y_enabled = Vec::new();
        let mut edge_x_disabled = Vec::new();
        let mut edge_y_disabled = Vec::new();

        for gene in &self.genes {
            if GeneType::from_u8(gene.flag[0]) != Some(GeneType::Synapse) {
                continue;
            }

            let synapse_status = if gene.flag[1] == SynapseStatus::Enabled as u8 {
                SynapseStatus::Enabled
            } else {
                SynapseStatus::Disabled
            };

            let source_id = gene.id;
            let destination_id = convert_f32_to_id(gene.local_data);

            if let (Some(&(x0, y0)), Some(&(x1, y1))) =
                (neuron_positions.get(&source_id), neuron_positions.get(&destination_id))
            {
                match synapse_status {
                    SynapseStatus::Enabled => {
                        edge_x_enabled.push(x0);
                        edge_x_enabled.push(x1);
                        edge_x_enabled.push(f64::NAN);
                        edge_y_enabled.push(y0);
                        edge_y_enabled.push(y1);
                        edge_y_enabled.push(f64::NAN);
                    }
                    SynapseStatus::Disabled => {
                        edge_x_disabled.push(x0);
                        edge_x_disabled.push(x1);
                        edge_x_disabled.push(f64::NAN);
                        edge_y_disabled.push(y0);
                        edge_y_disabled.push(y1);
                        edge_y_disabled.push(f64::NAN);
                    }
                }
            }
        }

        // Trace for enabled synapses
        let edge_trace_enabled = Scatter::new(edge_x_enabled, edge_y_enabled)
            .mode(Mode::Lines)
            .name("Enabled Synapses")
            .line(plotly::common::Line::new().color("#888").width(1.0))
            .hover_info(plotly::common::HoverInfo::None);

        plot.add_trace(edge_trace_enabled);

        // Trace for disabled synapses
        let edge_trace_disabled = Scatter::new(edge_x_disabled, edge_y_disabled)
            .mode(Mode::Lines)
            .name("Disabled Synapses")
            .line(
                plotly::common::Line::new()
                    .color("#ff0000")
                    .width(1.0)
                    .dash(plotly::common::DashType::Dash),
            )
            .hover_info(plotly::common::HoverInfo::None);

        plot.add_trace(edge_trace_disabled);

        // Collect neuron positions and types for node trace
        let mut node_x = Vec::new();
        let mut node_y = Vec::new();
        let mut node_text = Vec::new();
        let mut node_colors = Vec::new();
        let mut node_symbols = Vec::new();

        for gene in &self.genes {
            if GeneType::from_u8(gene.flag[0]) == Some(GeneType::Synapse) {
                continue;
            }

            let neuron_type = GeneType::from_u8(gene.flag[0]).unwrap();
            let (x, y) = neuron_positions[&gene.id];
            node_x.push(x);
            node_y.push(y);
            node_text.push(format!("ID: {:?}, Type: {:?}", gene.id, neuron_type));

            // Assign colors or symbols based on neuron type
            match neuron_type {
                GeneType::Input => {
                    node_colors.push("blue");
                    node_symbols.push(MarkerSymbol::Circle);
                }
                GeneType::Hidden => {
                    node_colors.push("green");
                    node_symbols.push(MarkerSymbol::Square);
                }
                GeneType::Output => {
                    node_colors.push("red");
                    node_symbols.push(MarkerSymbol::Diamond);
                }
                _ => {}
            }
        }

        // Node trace
        let node_trace = Scatter::new(node_x, node_y)
            .mode(Mode::Markers)
            .name("Neurons")
            .marker(
                plotly::common::Marker::new()
                    .size(10)
                    .color_array(node_colors)
                    .symbol(MarkerSymbol::Star),
            )
            .text_array(node_text)
            .hover_info(plotly::common::HoverInfo::Text);

        plot.add_trace(node_trace);

        plot.show();
    }
    pub fn display(&self) {
        println!("################ GENOME DISPLAY ################");
        for gene in &self.genes {
            let seed = gene.id[0];
            let id = gene.id[1];
            let flag1 = gene.flag[0];
            let flag2 = gene.flag[1];
            let lcl_data = gene.local_data;
            let ext_data = gene.extern_data;
            if flag1 == 1 {
                println!(
                    "INPUT NEURON - - - - - # ID: {:?} # CHILDREN: {} # BIAS: {}",
                    gene.id, flag2, lcl_data
                );
            }
            if flag1 == 2 {
                println!(
                    "L HIDDEN NEURON- - - - # ID: {:?} # CHILDREN: {} # BIAS: {}",
                    gene.id, flag2, lcl_data
                );
            }
            if flag1 == 3 {
                println!(
                    "OUTPUT NEURON- - - - - # ID: {:?} # CHILDREN: {} # BIAS: {}",
                    gene.id, flag2, lcl_data
                );
            }
            if flag1 == 10 {
                let mut enabled = "True";
                if flag2 == 11 {
                    enabled = "False";
                }
                let to_id = convert_f32_to_id(lcl_data);
                println!("  L SYNAPSE- - - - - - # ID_FROM {:?} # ID_TO {:?} # WEIGHT {} # ENABLED: [{}]", gene.id, to_id, ext_data, enabled)
            }
        }
        println!("################################################\n");
    }
    pub fn debug_display(&self) {
        println!("DEBUG DISPLAY");
        for gene in &self.genes {
            println!("{:?}", gene);
        }
    }
}