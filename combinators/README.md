# `Option<T>`

![option cheatsheet](./option.png)

---

# `Result<T, E>`

![result cheatsheet](./result.png)

# Markup

<style>
.precode {
  background: white;
  padding: 0px;
  margin: 0px;
}
</style>

<table>
  <tr>
    <th colspan="2"><code><span class="fnname">is_some</span>() -&gt; <span class="primitive">bool</span></code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">is_none</span>() -&gt; <span class="primitive">bool</span></code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="bool-val">false</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="bool-val">false</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="bool-val">true</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">unwrap</span>() -&gt; T</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">unwrap_or</span>(def) -&gt; T</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">unwrap_or_else</span>(f) -&gt; T</code></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="macro">panic</span><span class="macro">!</span>()</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="ident">def</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="ident">f</span>()</pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">expect</span>(msg) -&gt; T</code></th>
    <th></th>
    <th colspan="2"></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="macro">panic</span><span class="macro">!</span>(<span class="ident">msg</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">map</span>(f) -&gt; <span class="enum">Option</span>&lt;U&gt;</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">map_or</span>(default, f) -&gt; U</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">map_or_else</span>(default, f) -&gt; U</code></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">f</span>(<span class="ident">t</span>))</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">f</span>(<span class="ident">t</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">f</span>(<span class="ident">t</span>)</pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="ident">default</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="ident">default</span></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">and</span>(optb) -&gt; <span class="enum">Option</span>&lt;U&gt;</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">and_then</span>(f) -&gt; <span class="enum">Option</span>&lt;U&gt;</code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="ident">optb</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">f</span>(<span class="ident">t</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">or</span>(optb) -&gt; <span class="enum">Option</span>&lt;T&gt;</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">or_else</span>(f) -&gt; <span class="enum">Option</span>&lt;T&gt;</code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="self">self</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Some</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="self">self</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="ident">optb</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td><pre class="rust precode"><span class="ident">f</span>()</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
</table>

<table>
  <tr>
    <th colspan="2"><code><span class="fnname">is_ok</span>() -&gt; <span class="primitive">bool</span></code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">is_err</span>() -&gt; <span class="primitive">bool</span></code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="bool-val">true</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="bool-val">false</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="bool-val">false</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="bool-val">true</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">ok</span>() -&gt; <span class="enum">Option</span>&lt;T&gt;</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">err</span>() -&gt; <span class="enum">Option</span>&lt;E&gt;</code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">None</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">e</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">map</span>(op) -&gt; <span class="enum">Result</span>&lt;U,&nbsp;E&gt;</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">map_err</span>(op) -&gt; <span class="enum">Result</span>&lt;T,&nbsp;F&gt;</code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">op</span>(<span class="ident">t</span>))</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">op</span>(<span class="ident">e</span>))</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">and</span>(res) -&gt; <span class="enum">Result</a>&lt;U,&nbsp;E&gt;</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">and_then</span>(op) -&gt; <span class="enum">Result</a>&lt;U,&nbsp;E&gt;</code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="ident">res</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">op</span>(<span class="ident">t</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">or</span>(res) -&gt; <span class="enum">Result</span>&lt;T,&nbsp;F&gt;</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">or_else</span>(op) -&gt; <span class="enum">Result</span>&lt;T,&nbsp;F&gt;</code></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="ident">res</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">op</span>(<span class="ident">e</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">unwrap</span>() -&gt; T</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">unwrap_or</span>(optb) -&gt; T</code></th>
    <th></th>
    <th colspan="2"><code><span class="fnname">unwrap_or_else</span>(op) -&gt; T</code></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="macro">panic</span><span class="macro">!</span>(<span class="ident">e</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(_)</pre></td>
    <td><pre class="rust precode"><span class="ident">optb</span></pre></td>
    <td></td>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">op</span>(<span class="ident">e</span>)</pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">expect</span>(msg) -&gt; T</code></th>
    <th></th>
    <th colspan="2"></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">t</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="macro">panic</span><span class="macro">!</span>(<span class="ident">msg</span>, <span class="ident">e</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <th colspan="2"><code><span class="fnname">unwrap_err</span>() -&gt; E</code></th>
    <th></th>
    <th colspan="2"></th>
    <th></th>
    <th colspan="2"></th>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Ok</span>(<span class="ident">t</span>)</pre></td>
    <td><pre class="rust precode"><span class="macro">panic</span><span class="macro">!</span>(<span class="ident">t</span>)</pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td><pre class="rust precode"><span class="prelude-val">Err</span>(<span class="ident">e</span>)</pre></td>
    <td><pre class="rust precode"><span class="ident">e</span></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
    <td></td>
    <td><pre class="rust precode"></pre></td>
    <td><pre class="rust precode"></pre></td>
  </tr>
  <tr>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
</table>
