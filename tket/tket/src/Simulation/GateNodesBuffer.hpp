// Copyright 2019-2022 Cambridge Quantum Computing
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#pragma once

#include <memory>

#include "GateNode.hpp"
#include "Utils/MatrixAnalysis.hpp"

namespace tket {
namespace tket_sim {
namespace internal {

/** Pass the raw gate unitary matrix elements into this class one by one,
 *  but let this class perform any extra optimisation. (So, we leave open
 *  future options to optimise more aggressively when simulating).
 *
 *  E.g., if you had consecutive 1-qubit gates acting on the same qubit,
 *  it might be quicker to multiply the small 2x2 matrices together before
 *  expanding to a big (2^n)*(2^n) matrix.
 *
 *  If you had, e.g. a 2-qubit gate acting on {0,1} followed by another acting
 *  on {1,0}, it might be quicker to multiply the 4x4 matrices together,
 *  with an additional 4x4 lifted permutation generated by
 *  the qubit permutation {0,1} -> {1,0}.
 *
 *  If you had successive gates acting on qubits {0}, {1,2}, {0}, {3,4}, {2,1},
 *  the gates on {0} can be combined into one gate,
 *  and also the gates on {1,2} and {2,1}, because the sets {0}, {1,2}, {3,4}
 *  are disjoint. Thus, we might store data for multiple gates.
 *
 *  If you had gates acting on {0,1,2} and {2,0}, or {0,1,2} and {0,2,3},
 *  you might merge them, i.e. if two qubit index sets have a small symmetric
 *  difference than you can combine the small unitaries together before lifting
 *  and copying the entries.
 *
 *  Of course, this would all be simulating the exact same gates, just in a
 *  computationally more efficient way; allowing the gates themselves to be
 *  changed could give yet more speedup possibilities.
 */
class GateNodesBuffer {
 public:
  /** The full (2^n)*(2^n) unitaries of the nodes will be calculated,
   *  and left-multiply the matrix.
   *  The matrix must remain valid throughout the lifetime
   *  of this object, and the caller should not alter the matrix
   *  in any other way.
   *
   *  @param matrix The matrix representing the full unitary of the circuit
   *       arising the gates added in sequence, in ILO-BE convention.
   *  @param abs_epsilon Used to convert almost-zero entries to zero entries:
   *      any z with std::abs(z) <= abs_epsilon is treated as zero.
   */
  GateNodesBuffer(Eigen::MatrixXcd& matrix, double abs_epsilon);

  ~GateNodesBuffer();

  /** Process and store the next node, possibly with optimisation,
   *  e.g. combining gates etc. Note that this class might not update
   *  the original matrix immediately.
   */
  void push(const GateNode& node);

  void add_global_phase(double);

  /** The buffer might be cleared and optimised regularly, but the caller
   *  must call this at the end to ensure that any leftover nodes
   *  are also processed; updates the original matrix
   *  passed into the constructor.
   */
  void flush();

 private:
  // Pimpl idiom.
  struct Impl;
  std::unique_ptr<Impl> pimpl;
};

}  // namespace internal
}  // namespace tket_sim
}  // namespace tket