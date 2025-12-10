advent_of_code::solution!(10);

use std::collections::HashSet;

const EPSILON: f64 = 1e-10;
const SOLUTION_TOLERANCE: f64 = 0.01;
const MAX_SEARCH_ITERATIONS: usize = 10_000_000;

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    button_effects: Vec<Vec<usize>>,
    target_counters: Vec<usize>,
}

impl Machine {
    fn parse(line: &str) -> Option<Self> {
        let target_lights = Self::parse_target_lights(line)?;
        let button_effects = Self::parse_buttons(line)?;
        let target_counters = Self::parse_counters(line)?;

        Some(Machine {
            target_lights,
            button_effects,
            target_counters,
        })
    }

    fn parse_target_lights(line: &str) -> Option<Vec<bool>> {
        let start = line.find('[')?;
        let end = line.find(']')?;
        let lights_str = &line[start + 1..end];
        Some(lights_str.chars().map(|c| c == '#').collect())
    }

    fn parse_buttons(line: &str) -> Option<Vec<Vec<usize>>> {
        let target_end = line.find(']')?;
        let rest = &line[target_end + 1..];
        let mut buttons = Vec::new();
        let mut pos = 0;

        while let Some(start) = rest[pos..].find('(') {
            let start = pos + start;
            let end = rest[start..].find(')')? + start;
            let button_str = &rest[start + 1..end];

            let indices: Vec<usize> = button_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            if !indices.is_empty() {
                buttons.push(indices);
            }

            pos = end + 1;
        }

        Some(buttons)
    }

    fn parse_counters(line: &str) -> Option<Vec<usize>> {
        let start = line.find('{')?;
        let end = line.find('}')?;
        let counters_str = &line[start + 1..end];

        Some(
            counters_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
        )
    }

    // Part 1: Light toggle problem (XOR logic)
    fn min_light_presses(&self) -> usize {
        let num_buttons = self.button_effects.len();
        let num_lights = self.target_lights.len();
        let mut min_presses = usize::MAX;

        // Try all 2^n combinations (each button pressed 0 or 1 times)
        for mask in 0u32..(1 << num_buttons) {
            let mut lights = vec![false; num_lights];
            let presses = mask.count_ones() as usize;

            for (button_idx, button_effects) in self.button_effects.iter().enumerate() {
                if mask & (1 << button_idx) != 0 {
                    for &light_idx in button_effects {
                        if light_idx < num_lights {
                            lights[light_idx] = !lights[light_idx];
                        }
                    }
                }
            }

            if lights == self.target_lights {
                min_presses = min_presses.min(presses);
            }
        }

        min_presses
    }

    // Part 2: Counter increment problem (integer linear programming)
    fn min_counter_presses(&self) -> Option<usize> {
        LinearSolver::new(self).solve()
    }
}

// Linear programming solver for Part 2
struct LinearSolver<'a> {
    machine: &'a Machine,
    num_buttons: usize,
    num_counters: usize,
}

impl<'a> LinearSolver<'a> {
    fn new(machine: &'a Machine) -> Self {
        Self {
            machine,
            num_buttons: machine.button_effects.len(),
            num_counters: machine.target_counters.len(),
        }
    }

    fn solve(&self) -> Option<usize> {
        let matrix = self.build_augmented_matrix();
        let (reduced_matrix, pivot_cols) = self.gaussian_elimination(matrix);
        let free_vars = self.identify_free_variables(&pivot_cols);

        if free_vars.is_empty() {
            return self.extract_solution(&reduced_matrix, &pivot_cols, &[], &[]);
        }

        self.optimize_free_variables(&reduced_matrix, &pivot_cols, &free_vars)
    }

    fn build_augmented_matrix(&self) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; self.num_buttons + 1]; self.num_counters];

        for (counter_idx, &target_val) in self.machine.target_counters.iter().enumerate() {
            for (button_idx, button) in self.machine.button_effects.iter().enumerate() {
                if button.contains(&counter_idx) {
                    matrix[counter_idx][button_idx] = 1.0;
                }
            }
            matrix[counter_idx][self.num_buttons] = target_val as f64;
        }

        matrix
    }

    fn gaussian_elimination(&self, mut matrix: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<usize>) {
        let mut pivot_cols = Vec::new();
        let mut current_row = 0;

        for col in 0..self.num_buttons {
            // Find row with largest absolute value in this column (partial pivoting)
            let pivot_row = (current_row..self.num_counters)
                .max_by(|&a, &b| {
                    matrix[a][col]
                        .abs()
                        .partial_cmp(&matrix[b][col].abs())
                        .unwrap()
                })
                .unwrap();

            if matrix[pivot_row][col].abs() < EPSILON {
                continue; // Skip zero columns
            }

            matrix.swap(current_row, pivot_row);
            pivot_cols.push(col);

            // Normalize pivot row
            let pivot = matrix[current_row][col];
            for j in col..=self.num_buttons {
                matrix[current_row][j] /= pivot;
            }

            // Eliminate column in all other rows
            for row in 0..self.num_counters {
                if row != current_row && matrix[row][col].abs() > EPSILON {
                    let factor = matrix[row][col];
                    for j in col..=self.num_buttons {
                        matrix[row][j] -= factor * matrix[current_row][j];
                    }
                }
            }

            current_row += 1;
            if current_row >= self.num_counters {
                break;
            }
        }

        (matrix, pivot_cols)
    }

    fn identify_free_variables(&self, pivot_cols: &[usize]) -> Vec<usize> {
        let pivot_set: HashSet<_> = pivot_cols.iter().copied().collect();
        (0..self.num_buttons)
            .filter(|col| !pivot_set.contains(col))
            .collect()
    }

    fn optimize_free_variables(
        &self,
        matrix: &[Vec<f64>],
        pivot_cols: &[usize],
        free_vars: &[usize],
    ) -> Option<usize> {
        let bounds = self.compute_free_variable_bounds(matrix, free_vars);
        let mut search_state = OptimizationState::new();

        self.search_free_variables(
            matrix,
            pivot_cols,
            free_vars,
            &bounds,
            &mut Vec::new(),
            &mut search_state,
        );

        search_state.best_cost()
    }

    fn compute_free_variable_bounds(&self, matrix: &[Vec<f64>], free_vars: &[usize]) -> Vec<usize> {
        let max_target = *self.machine.target_counters.iter().max().unwrap_or(&0);

        free_vars
            .iter()
            .map(|&free_col| {
                let constraint_bound = matrix
                    .iter()
                    .filter_map(|row| {
                        let coeff = row[free_col];
                        if coeff.abs() > EPSILON {
                            Some((row[self.num_buttons] / coeff).abs().ceil() as usize)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0);

                constraint_bound.min(max_target)
            })
            .collect()
    }

    fn search_free_variables(
        &self,
        matrix: &[Vec<f64>],
        pivot_cols: &[usize],
        free_vars: &[usize],
        bounds: &[usize],
        current_values: &mut Vec<usize>,
        state: &mut OptimizationState,
    ) {
        if state.should_terminate() {
            return;
        }

        if current_values.len() == free_vars.len() {
            if let Some(cost) = self.extract_solution(matrix, pivot_cols, free_vars, current_values)
            {
                state.update_best(cost);
            }
            return;
        }

        let depth = current_values.len();
        let current_sum: usize = current_values.iter().sum();

        if state.should_prune(current_sum) {
            return;
        }

        let max_val = bounds[depth].min(state.remaining_budget(current_sum));

        for val in 0..=max_val {
            current_values.push(val);

            self.search_free_variables(
                matrix,
                pivot_cols,
                free_vars,
                bounds,
                current_values,
                state,
            );

            current_values.pop();

            if state.can_terminate_early(current_sum + val) {
                break;
            }
        }
    }

    fn extract_solution(
        &self,
        matrix: &[Vec<f64>],
        pivot_cols: &[usize],
        free_vars: &[usize],
        free_values: &[usize],
    ) -> Option<usize> {
        let mut solution = vec![0.0; self.num_buttons];

        // Set free variable values
        for (i, &free_col) in free_vars.iter().enumerate() {
            solution[free_col] = free_values[i] as f64;
        }

        // Compute pivot variable values from constraints
        for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
            let mut rhs = matrix[row_idx][self.num_buttons];

            // Subtract contributions from free variables
            for (i, &free_col) in free_vars.iter().enumerate() {
                rhs -= matrix[row_idx][free_col] * free_values[i] as f64;
            }

            solution[pivot_col] = rhs;
        }

        if !self.is_valid_solution(&solution) {
            return None;
        }

        Some(solution.iter().map(|&v| v.round() as usize).sum())
    }

    fn is_valid_solution(&self, solution: &[f64]) -> bool {
        // Check all values are non-negative integers
        if !solution.iter().all(|&val| {
            val >= -SOLUTION_TOLERANCE && (val - val.round()).abs() <= SOLUTION_TOLERANCE
        }) {
            return false;
        }

        // Verify all constraints are satisfied
        for (counter_idx, &target) in self.machine.target_counters.iter().enumerate() {
            let sum: usize = self
                .machine
                .button_effects
                .iter()
                .enumerate()
                .filter(|(_, button)| button.contains(&counter_idx))
                .map(|(button_idx, _)| solution[button_idx].round() as usize)
                .sum();

            if sum != target {
                return false;
            }
        }

        true
    }
}

// Optimization state tracking
struct OptimizationState {
    best_cost: usize,
    iterations: usize,
}

impl OptimizationState {
    fn new() -> Self {
        Self {
            best_cost: usize::MAX,
            iterations: 0,
        }
    }

    fn update_best(&mut self, cost: usize) {
        self.best_cost = self.best_cost.min(cost);
    }

    fn should_terminate(&mut self) -> bool {
        self.iterations += 1;
        self.iterations > MAX_SEARCH_ITERATIONS
    }

    fn should_prune(&self, current_sum: usize) -> bool {
        current_sum >= self.best_cost
    }

    fn remaining_budget(&self, current_sum: usize) -> usize {
        self.best_cost.saturating_sub(current_sum)
    }

    fn can_terminate_early(&self, current_sum: usize) -> bool {
        self.best_cost < usize::MAX && self.best_cost <= current_sum
    }

    fn best_cost(&self) -> Option<usize> {
        if self.best_cost == usize::MAX {
            None
        } else {
            Some(self.best_cost)
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let total: usize = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(Machine::parse)
        .map(|machine| machine.min_light_presses())
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let total: usize = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(Machine::parse)
        .filter_map(|machine| machine.min_counter_presses())
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
