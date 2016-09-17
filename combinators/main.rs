//! <style>
//! .precode {
//!   background: white;
//!   padding: 0px;
//!   margin: 0px;
//! }
//! </style>
//! 
//! <table style="border: none;">
//!   <tr>
//!     <th colspan="2"><code><span class="fnname">is_some</span>() -&gt; <span class="primitive">bool</span></code></th>
//!     <th></th>
//!     <th colspan="2"><code><span class="fnname">is_none</span>() -&gt; <span class="primitive">bool</span></code></th>
//!     <th></th>
//!     <th colspan="2"></th>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
//!     <td><pre class="rust precode"><span class="bool-val">false</span></pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="bool-val">false</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="bool-val">true</span></pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <th colspan="2"><code><span class="fnname">unwrap</span>() -&gt; T</code></th>
//!     <th></th>
//!     <th colspan="2"><code><span class="fnname">unwrap_or</span>(def) -&gt; T</code></th>
//!     <th></th>
//!     <th colspan="2"><code><span class="fnname">unwrap_or_else</span>(f) -&gt; T</code></th>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">t</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">t</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">t</span></pre></td>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="macro">panic</span><span class="macro">!</span>()</pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="ident">def</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="ident">f</span>()</pre></td>
//!   </tr>
//!   <tr>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <th colspan="2"><code><span class="fnname">expect</span>(msg) -&gt; T</code></th>
//!     <th></th>
//!     <th colspan="2"></th>
//!     <th></th>
//!     <th colspan="2"></th>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">t</span></pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="macro">panic</span><span class="macro">!</span>(<span class="ident">msg</span>)</pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <th colspan="2"><code><span class="fnname">map</span>(f) -&gt; <span class="enum">Option</span>&lt;U&gt;</code></th>
//!     <th></th>
//!     <th colspan="2"><code><span class="fnname">map_or</span>(default, f) -&gt; U</code></th>
//!     <th></th>
//!     <th colspan="2"><code><span class="fnname">map_or_else</span>(default, f) -&gt; U</code></th>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">f</span>(<span class="ident">t</span>))</pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">f</span>(<span class="ident">t</span>)</pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">f</span>(<span class="ident">t</span>)</pre></td>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="ident">default</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="ident">default</span></pre></td>
//!   </tr>
//!   <tr>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <th colspan="2"><code><span class="fnname">and</span>(optb) -&gt; <span class="enum">Option</span>&lt;U&gt;</code></th>
//!     <th></th>
//!     <th colspan="2"><code><span class="fnname">and_then</span>(f) -&gt; <span class="enum">Option</span>&lt;U&gt;</code></th>
//!     <th></th>
//!     <th colspan="2"></th>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">optb</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
//!     <td><pre class="rust precode"><span class="ident">f</span>(<span class="ident">t</span>)</pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <th colspan="2"><code><span class="fnname">or</span>(optb) -&gt; <span class="enum">Option</span>&lt;T&gt;</code></th>
//!     <th></th>
//!     <th colspan="2"><code><span class="fnname">or_else</span>(f) -&gt; <span class="enum">Option</span>&lt;T&gt;</code></th>
//!     <th></th>
//!     <th colspan="2"></th>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
//!     <td><pre class="rust precode"><span class="self">self</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
//!     <td><pre class="rust precode"><span class="self">self</span></pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="ident">optb</span></pre></td>
//!     <td></td>
//!     <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
//!     <td><pre class="rust precode"><span class="ident">f</span>()</pre></td>
//!     <td></td>
//!     <td></td>
//!     <td></td>
//!   </tr>
//! </table>

fn main() { }
