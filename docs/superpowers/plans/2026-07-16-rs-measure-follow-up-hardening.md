# rs-measure Follow-up Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 修复内建比例规范化缺口，并把单位解析、uom、Serde、测试路径和 inline 契约固化为可执行回归测试。

**Architecture:** 生产代码只增加 crate 内 const 比例构造和解析边界校验，不改变公共路径。测试侧用具体 family 展开的宏读取 uom Quantity 的 SI 基准 value，并用两个独立下游 fixture crate 验证 feature 命名空间。测试目录严格镜像 src，纯别名通过共享测试宏保持最小化。

**Tech Stack:** Rust 1.94、rust_decimal 1.39、serde/serde_json、uom 0.38、Cargo、Bash、项目 rs-ci 包装脚本。

## Global Constraints

- 保留 crate 名 `qubit-measure`、edition 2024、MSRV 1.94 和默认空 feature 集。
- 不改变现有公共类型、枚举变体、方法名、模块路径或 JSON 三字段格式。
- 所有 Rust 测试放在 tests/，禁止 src 内联测试。
- 每个具体源码文件严格映射到 tests 下同路径的 `_tests.rs` 文件。
- 所有新增 Rust 文件复制 src/lib.rs 的完整 Apache-2.0 文件头。
- 生产行为变更必须先看到聚焦测试因目标缺口失败，再写最小实现。
- 不修改共享 `.rs-ci` 目录；项目定制行为只改根包装脚本和根 fixtures/。
- 未经用户明确授权，不执行 git add、git commit、git push；本计划中的检查点只查看 diff/status。

---

### Task 1: 约分所有内建 ConversionFactor

**Files:**
- Create: `tests/consts_tests.rs`
- Modify: `tests/mod.rs`
- Modify: `src/measure/conversion_factor.rs`
- Modify: `src/consts.rs`
- Modify: `tests/measure/support/definition_assertions.rs`
- Modify: `tests/measure/units/angle_tests.rs`
- Modify: `tests/measure/units/angular_velocity_tests.rs`
- Modify: `tests/measure/units/solid_angle_tests.rs`

**Interfaces:**
- Produces: `pub(crate) const fn ConversionFactor::from_const_integers(i128, i128) -> ConversionFactor`。
- Preserves: `ConversionFactor::new(Decimal, Decimal) -> Result<ConversionFactor, MeasurementError>`。

- [ ] **Step 1: 写内建因子失败测试**

在 `tests/consts_tests.rs` 写入文件头后增加：

```rust
//! Exact built-in conversion-factor tests.

use qubit_measure::{
    ConversionFactor,
    ConversionOptions,
    Decimal,
    Unit,
    UnitDefinition,
    unit,
};
use rust_decimal::dec;

#[test]
fn test_builtin_revolution_factor_uses_reduced_terms() {
    let builtin = unit::Angle::Revolution
        .definition()
        .expect("revolution definition should be valid")
        .factor();
    let normalized = ConversionFactor::new(
        dec!(3141592653589793),
        dec!(500000000000000),
    )
    .expect("normalized revolution factor should be valid");

    assert_eq!(builtin, normalized);
}

#[test]
fn test_builtin_revolution_matches_equivalent_external_definition_at_max() {
    let builtin = unit::Angle::Revolution
        .definition()
        .expect("revolution definition should be valid");
    let normalized = UnitDefinition::new(
        ConversionFactor::new(
            dec!(3141592653589793),
            dec!(500000000000000),
        )
        .expect("normalized revolution factor should be valid"),
        Decimal::ZERO,
    );

    assert_eq!(
        builtin
            .convert_value_to(
                Decimal::MAX,
                normalized,
                ConversionOptions::default(),
            )
            .expect("equivalent definitions should preserve Decimal::MAX"),
        Decimal::MAX,
    );
}
```

在 `tests/mod.rs` 增加 `mod consts_tests;`。

- [ ] **Step 2: 运行聚焦测试并确认 RED**

Run:

```bash
cargo +1.94.0 test --test mod test_builtin_revolution -- --nocapture
```

Expected: `test_builtin_revolution_factor_uses_reduced_terms` 因 `6283185307179586 / 1000000000000000` 不等于约分形式而 FAIL；极值测试不得因导入或编译错误失败。

- [ ] **Step 3: 实现 crate 内 const 约分构造器**

在 `ConversionFactor` 的构造器组中、公开的 `from_integer` 之后和 getter 之前增加：

```rust
/// Creates a reduced conversion factor from positive integer terms in const
/// contexts.
///
/// # Arguments
///
/// * `numerator` - Positive integer numerator.
/// * `denominator` - Positive integer denominator.
///
/// # Returns
///
/// A factor whose integer terms have no common divisor.
///
/// # Panics
///
/// Panics if either term is non-positive or a reduced term exceeds Decimal's
/// 96-bit coefficient range.
pub(crate) const fn from_const_integers(
    numerator: i128,
    denominator: i128,
) -> Self {
    assert!(numerator > 0);
    assert!(denominator > 0);
    let divisor = greatest_common_divisor(numerator, denominator);
    Self {
        numerator: decimal_from_positive_integer(numerator / divisor),
        denominator: decimal_from_positive_integer(denominator / divisor),
    }
}
```

把 `greatest_common_divisor` 改为 `const fn`，并在同文件增加完整 Rustdoc 的：

```rust
const fn decimal_from_positive_integer(value: i128) -> Decimal {
    assert!(value > 0);
    let magnitude = value as u128;
    assert!(magnitude >> 96 == 0);
    Decimal::from_parts(
        magnitude as u32,
        (magnitude >> 32) as u32,
        (magnitude >> 64) as u32,
        false,
        0,
    )
}
```

在 `definition!` 中用以下逻辑替换字段直构造：

```rust
let factor = crate::measure::ConversionFactor::from_const_integers(
    numerator,
    denominator,
);
crate::measure::UnitDefinition::new(
    factor,
    decimal!($offset_mantissa, $offset_scale),
)
```

- [ ] **Step 4: 更新 golden oracle 并增强全量不变量**

更新精确文本：

```text
Angle::Revolution                    3141592653589793 / 500000000000000
AngularVelocity::RevolutionPerSecond 3141592653589793 / 500000000000000
AngularVelocity::RevolutionPerMinute 3141592653589793 / 30000000000000000
SolidAngle::Spat                     3141592653589793 / 250000000000000
```

在 `assert_definition_cases` 每个 case 中重建并比较规范因子：

```rust
let normalized = qubit_measure::ConversionFactor::new(
    definition.factor().numerator(),
    definition.factor().denominator(),
)
.expect("built-in factor should be positive");
assert_eq!(definition.factor(), normalized);
```

- [ ] **Step 5: 运行 GREEN 与相关回归**

Run:

```bash
cargo +1.94.0 test --test mod test_builtin_revolution -- --nocapture
cargo +1.94.0 test --test mod definitions_match_exact_golden_values -- --nocapture
cargo +1.94.0 test --test mod decimal_conversion -- --nocapture
```

Expected: 三条命令 exit 0，极值测试精确返回 Decimal::MAX。

- [ ] **Step 6: 检查差异，不提交**

Run: `git --no-pager diff -- src/consts.rs src/measure/conversion_factor.rs tests/consts_tests.rs tests/measure/support/definition_assertions.rs tests/measure/units`

Expected: 仅包含 const 约分实现、4 个 oracle 更新及对应测试。

---

### Task 2: 收紧元数据并保持特殊前缀的空格分隔往返

**Files:**
- Modify: `src/private.rs`
- Modify: `src/measure/unit.rs`
- Modify: `src/measure/measurement.rs`
- Modify: `src/measure/units.rs`
- Modify: `tests/measure/fixtures/custom_length.rs`
- Modify: `tests/measure/external_unit_tests.rs`
- Modify: `tests/measure/measurement_tests.rs`
- Modify: `tests/measure/private_tests.rs`（Task 6 再移动到根路径）
- Modify: `README.md`
- Modify: `README.zh_CN.md`

**Interfaces:**
- Preserves: Unit parsers trim caller input。
- Changes: macro metadata rejects leading/trailing Unicode White_Space。
- Changes: spaced Measurement accepts unit suffix beginning with `.`, `+`, `-`; compact form remains strict。

- [ ] **Step 1: 写解析 RED 测试**

给 `CustomLength` 增加：

```rust
/// Signed custom unit used to verify spaced measurement parsing.
Signed => {
    symbol: "+cu";
    coefficient: 1;
}
```

在 `external_unit_tests.rs` 增加：

```rust
#[test]
fn test_spaced_measurement_round_trips_reserved_unit_prefix() {
    let measurement = Measurement::new(dec!(1.25), CustomLength::Signed);
    let text = measurement.to_string();

    assert_eq!(text, "1.25 +cu");
    assert_eq!(text.parse::<Measurement<CustomLength>>(), Ok(measurement));
    assert_eq!(
        serde_json::from_value::<Measurement<CustomLength>>(
            serde_json::to_value(measurement)
                .expect("measurement should serialize"),
        )
        .expect("measurement should deserialize"),
        measurement,
    );
}
```

- [ ] **Step 2: 运行解析测试并确认 RED**

Run: `cargo +1.94.0 test --test mod test_spaced_measurement_round_trips_reserved_unit_prefix -- --nocapture`

Expected: FromStr 返回 InvalidMeasurement，数值或 fixture 编译不得失败。

- [ ] **Step 3: 写元数据 compile_fail RED doctest**

在 `define_unit_family!` Rustdoc 增加两个 `compile_fail` 示例：

```rust
use qubit_measure::define_unit_family;

define_unit_family! {
    /// Invalid family with surrounding canonical whitespace.
    enum WhitespaceSymbolUnit for "whitespace_symbol_unit" {
        /// Invalid canonical owner.
        Invalid => {
            symbol: " m";
            coefficient: 1;
        }
    }
}
```

```rust
use qubit_measure::define_unit_family;

define_unit_family! {
    /// Invalid family with surrounding alias whitespace.
    enum WhitespaceAliasUnit for "whitespace_alias_unit" {
        /// Invalid alias owner.
        Invalid => {
            symbol: "m";
            coefficient: 1;
            aliases: ["meter "];
        }
    }
}
```

Run: `cargo +1.94.0 test --doc define_unit_family -- --nocapture`

Expected: 两个新增 compile_fail 示例因为当前代码错误地编译成功而导致 doctest FAIL。

- [ ] **Step 4: 实现 Unicode 边界空白 const 校验**

在 `private.rs` 增加以下 const helper；它们按 UTF-8 字节精确覆盖：

```text
U+0009..U+000D, U+0020, U+0085, U+00A0, U+1680,
U+2000..U+200A, U+2028, U+2029, U+202F, U+205F, U+3000
```

```rust
/// Reports whether text starts with a Unicode White_Space character.
///
/// # Arguments
///
/// * `value` - Text whose first scalar value is inspected.
///
/// # Returns
///
/// `true` when the first scalar has the Unicode White_Space property.
const fn has_leading_unit_whitespace(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if matches!(bytes[0], b'\t'..=b'\r' | b' ') {
        return true;
    }
    if bytes.len() >= 2
        && bytes[0] == 0xC2
        && matches!(bytes[1], 0x85 | 0xA0)
    {
        return true;
    }
    if bytes.len() >= 3 {
        return (bytes[0] == 0xE1
            && bytes[1] == 0x9A
            && bytes[2] == 0x80)
            || (bytes[0] == 0xE2
                && bytes[1] == 0x80
                && matches!(bytes[2], 0x80..=0x8A | 0xA8 | 0xA9 | 0xAF))
            || (bytes[0] == 0xE2
                && bytes[1] == 0x81
                && bytes[2] == 0x9F)
            || (bytes[0] == 0xE3
                && bytes[1] == 0x80
                && bytes[2] == 0x80);
    }
    false
}

/// Reports whether text ends with a Unicode White_Space character.
///
/// # Arguments
///
/// * `value` - Text whose final scalar value is inspected.
///
/// # Returns
///
/// `true` when the final scalar has the Unicode White_Space property.
const fn has_trailing_unit_whitespace(value: &str) -> bool {
    let bytes = value.as_bytes();
    let length = bytes.len();
    if length == 0 {
        return false;
    }
    if matches!(bytes[length - 1], b'\t'..=b'\r' | b' ') {
        return true;
    }
    if length >= 2
        && bytes[length - 2] == 0xC2
        && matches!(bytes[length - 1], 0x85 | 0xA0)
    {
        return true;
    }
    if length >= 3 {
        return (bytes[length - 3] == 0xE1
            && bytes[length - 2] == 0x9A
            && bytes[length - 1] == 0x80)
            || (bytes[length - 3] == 0xE2
                && bytes[length - 2] == 0x80
                && matches!(
                    bytes[length - 1],
                    0x80..=0x8A | 0xA8 | 0xA9 | 0xAF
                ))
            || (bytes[length - 3] == 0xE2
                && bytes[length - 2] == 0x81
                && bytes[length - 1] == 0x9F)
            || (bytes[length - 3] == 0xE3
                && bytes[length - 2] == 0x80
                && bytes[length - 1] == 0x80);
    }
    false
}
```

在 symbols 和 aliases 循环中分别加入：

```rust
assert!(
    !has_leading_unit_whitespace(symbol)
        && !has_trailing_unit_whitespace(symbol),
    "canonical unit symbol must not contain surrounding whitespace",
);
```

alias 使用错误文本 `unit alias must not contain surrounding whitespace`。在 `assert_unit_family_valid` 的运行时循环中用 `symbol.trim() == symbol` 和 `alias.trim() == alias` 明确执行同一契约。

- [ ] **Step 5: 修复 Measurement 分词边界**

把 `split_measurement_parts` 的核心逻辑改为：

```rust
let (value_text, unit_suffix) = trimmed.split_at(value_len);
let is_separated = unit_suffix.trim_start().len() != unit_suffix.len();
let unit_text = unit_suffix.trim();
if unit_text.is_empty()
    || (!is_separated && unit_text.starts_with(['.', '+', '-']))
{
    None
} else {
    Some((value_text, unit_text))
}
```

同步更新函数 Rustdoc，明确 compact/spaced 差异。

- [ ] **Step 6: 更新公共契约文档**

在 Unit、define_unit_family、README 英文和中文外部扩展章节中加入“canonical symbols and aliases must not contain leading or trailing Unicode whitespace”，不改变 canonical 优先级描述。

- [ ] **Step 7: 运行 GREEN 与保留边界测试**

Run:

```bash
cargo +1.94.0 test --test mod test_spaced_measurement_round_trips_reserved_unit_prefix -- --nocapture
cargo +1.94.0 test --test mod measurement_from_str -- --nocapture
cargo +1.94.0 test --doc define_unit_family -- --nocapture
```

Expected: 全部 exit 0；`1+2`、`1.2.3m` 等现有非法紧凑输入仍失败。

---

### Task 3: 用 SI 基准 value 验证全部 uom 映射

**Files:**
- Modify: `tests/measure/uom_unit_tests.rs`

**Interfaces:**
- Consumes: `UomUnit::Quantity` 的具体 uom 类型及其公开 `value: f64`。
- Produces: 每个内建 family 的双向独立 SI 映射检查。

- [ ] **Step 1: 用具体 family 测试宏替换自洽 helper**

删除 `assert_all_unit_variants_bridge_uom<U>`，增加：

```rust
macro_rules! assert_unit_family_matches_uom_base {
    ($unit:ty) => {{
        let base_unit = <$unit>::all()
            .iter()
            .copied()
            .find(|unit| {
                unit.definition()
                    .expect("unit definition should be valid")
                    == UnitDefinition::base()
            })
            .expect("unit family should contain exactly one base definition");

        for unit in <$unit>::all() {
            let source = Measurement::<$unit>::new(Decimal::ONE, *unit);
            let exact_base = source
                .convert_to(base_unit)
                .expect("Decimal conversion to the base unit should succeed");
            let expected_base = exact_base
                .value
                .to_f64()
                .expect("exact base value should fit f64 for the oracle");
            let quantity = source.to_uom_approx();

            assert_approx_eq(quantity.value, expected_base);

            let mut independent_base =
                Measurement::<$unit>::new(Decimal::ZERO, base_unit)
                    .to_uom_approx();
            independent_base.value = expected_base;
            let round_trip = Measurement::<$unit>::from_uom_approx(
                independent_base,
                *unit,
            )
            .expect("independent SI base value should convert to the unit");
            assert_approx_eq(
                round_trip
                    .value
                    .to_f64()
                    .expect("round-trip Decimal should fit f64"),
                1.0,
            );
        }
    }};
}
```

添加 `rust_decimal::prelude::ToPrimitive` 和 `UnitDefinition` 导入。把 `test_all_supported_unit_variants_bridge_through_uom` 内现有 56 个 helper 调用逐一替换为同名宏调用，family 列表保持完整。

- [ ] **Step 2: 运行新 oracle**

Run: `cargo +1.94.0 test --no-default-features --features uom --test mod test_all_supported_unit_variants_bridge_through_uom -- --nocapture`

Expected: 当前正确映射下 exit 0，宏在每个具体 family 展开后可访问关联 uom Quantity 的公开 `value` 字段。

- [ ] **Step 3: 证明测试能捕获错误映射**

用 apply_patch 临时把 `src/measure/units/length.rs` 的 Centimeter uom 类型从 `centimeter` 改为 `millimeter`，运行上一步命令。

Expected: 数值断言 FAIL，并显示 centimeter 的 SI base value 相差 10 倍。

立即用 apply_patch 恢复 `centimeter`，再次运行同一命令。

Expected: exit 0。临时变异不得出现在最终 diff。

- [ ] **Step 4: 运行全部 uom 测试**

Run: `cargo +1.94.0 test --no-default-features --features uom --test mod uom_unit_tests -- --nocapture`

Expected: exit 0，BTU 和 mmHg 独立 oracle 保持通过。

---

### Task 4: 增加真正的下游 feature fixture

**Files:**
- Create: `fixtures/downstream-default/Cargo.toml`
- Create: `fixtures/downstream-default/Cargo.lock`
- Create: `fixtures/downstream-default/src/lib.rs`
- Create: `fixtures/downstream-uom/Cargo.toml`
- Create: `fixtures/downstream-uom/Cargo.lock`
- Create: `fixtures/downstream-uom/src/lib.rs`
- Modify: `align-ci.sh`
- Modify: `ci-check.sh`

**Interfaces:**
- Default fixture has no uom dependency or feature。
- uom fixture enables dependency feature `qubit-measure/uom` but declares no local `uom` feature。

- [ ] **Step 1: 创建默认 fixture**

`fixtures/downstream-default/Cargo.toml`：

```toml
[package]
name = "qubit-measure-downstream-default-fixture"
version = "0.0.0"
edition = "2024"
rust-version = "1.94"
publish = false

[dependencies]
qubit-measure = { path = "../.." }

[workspace]
```

`src/lib.rs` 使用完整文件头并写入：

```rust
//! Default-feature downstream macro fixture.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Unit family whose unresolved uom tokens must be discarded.
    pub enum DefaultFixtureUnit for "default_fixture", uom = missing::Quantity {
        /// Base fixture unit.
        Base => {
            symbol: "dfu";
            coefficient: 1;
            uom: missing::BaseUnit;
        }
    }
}
```

Run:

```bash
CARGO_TARGET_DIR=target/downstream-fixtures \
    cargo +1.94.0 generate-lockfile \
    --manifest-path fixtures/downstream-default/Cargo.toml
CARGO_TARGET_DIR=target/downstream-fixtures \
    cargo +1.94.0 check --locked \
    --manifest-path fixtures/downstream-default/Cargo.toml
```

Expected: exit 0，证明 uom token 未参与默认构建名称解析。

- [ ] **Step 2: 创建 uom fixture**

`fixtures/downstream-uom/Cargo.toml`：

```toml
[package]
name = "qubit-measure-downstream-uom-fixture"
version = "0.0.0"
edition = "2024"
rust-version = "1.94"
publish = false

[dependencies]
qubit-measure = { path = "../..", features = ["uom"] }
uom = { version = "0.38", default-features = false, features = ["f64", "si", "std"] }

[workspace]
```

`src/lib.rs` 使用完整文件头并写入：

```rust
//! Uom-enabled downstream macro fixture without a local `uom` feature.

use qubit_measure::{
    UomUnit,
    define_unit_family,
};
use uom::si::f64::Length as UomLength;
use uom::si::length::meter;

define_unit_family! {
    /// External length family used to verify dependency-owned features.
    pub enum DownstreamLength for "downstream_length", uom = UomLength {
        /// Meter fixture unit.
        Meter => {
            symbol: "m";
            coefficient: 1;
            uom: meter;
        }
    }
}

/// Requires the generated downstream family to implement `UomUnit`.
pub fn assert_uom_bridge_is_generated()
where
    DownstreamLength: UomUnit,
{
}
```

Run:

```bash
CARGO_TARGET_DIR=target/downstream-fixtures \
    cargo +1.94.0 generate-lockfile \
    --manifest-path fixtures/downstream-uom/Cargo.toml
CARGO_TARGET_DIR=target/downstream-fixtures \
    cargo +1.94.0 check --locked \
    --manifest-path fixtures/downstream-uom/Cargo.toml
```

Expected: exit 0，无 unexpected_cfg 警告，trait bound 成立。

- [ ] **Step 3: 证明 uom fixture 捕获下游 cfg 回归**

用 apply_patch 临时给启用分支生成的 `impl UomUnit` 外包一层 `#[cfg(feature = "uom")]`，运行 uom fixture check。

Expected: fixture 自身没有 `uom` feature，出现 cfg 警告并因 DownstreamLength 未实现 UomUnit 而 FAIL。

立即恢复临时变异并重新运行，Expected: exit 0。

- [ ] **Step 4: 接入项目包装脚本**

在 `ci-check.sh` 调用共享 rs-ci 前设置
`BUILD_TOOLCHAIN="${RS_CI_BUILD_TOOLCHAIN:-1.94.0}"` 和
`CARGO_TARGET_DIR="$PROJECT_ROOT/target/downstream-fixtures"`，然后对两个 manifest 依次执行
`cargo +"$BUILD_TOOLCHAIN" check --locked --manifest-path ...`，成功后再执行：

```bash
exec env RS_CI_PROJECT_ROOT="$PROJECT_ROOT" \
    "$PROJECT_ROOT/.rs-ci/ci-check.sh" "$@"
```

在 `align-ci.sh` 中先执行共享 align（去掉 exec，保留退出码），再用 `${RS_CI_FMT_TOOLCHAIN:-nightly-2026-06-05}` 对两个 fixture manifest 执行：

```bash
cargo +"$FMT_TOOLCHAIN" fmt \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-default/Cargo.toml" \
    -- --config-path "$PROJECT_ROOT/.rs-ci/rustfmt.toml"
cargo +"$FMT_TOOLCHAIN" fmt \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-uom/Cargo.toml" \
    -- --config-path "$PROJECT_ROOT/.rs-ci/rustfmt.toml"
```

Run:

```bash
bash -n align-ci.sh ci-check.sh
CARGO_TARGET_DIR=target/downstream-fixtures cargo +1.94.0 check --locked --manifest-path fixtures/downstream-default/Cargo.toml
CARGO_TARGET_DIR=target/downstream-fixtures cargo +1.94.0 check --locked --manifest-path fixtures/downstream-uom/Cargo.toml
```

Expected: 全部 exit 0。

---

### Task 5: 锁定 Measurement Serde 错误边界

**Files:**
- Create: `tests/measure/internal/mod.rs`
- Create: `tests/measure/internal/measurement_wire_tests.rs`
- Modify: `tests/measure/mod.rs`

**Interfaces:**
- Tests public `Measurement<unit::Length>` behavior only；不扩大 MeasurementWire 可见性。

- [ ] **Step 1: 增加表驱动边界测试**

在 `measurement_wire_tests.rs` 使用完整文件头，导入 `qubit_measure::measurement` 和 `serde_json::json`，增加：

```rust
#[test]
fn test_measurement_wire_rejects_missing_required_fields() {
    let cases = [
        json!({"value": "1", "unit": "m"}),
        json!({"quantity": "length", "unit": "m"}),
        json!({"quantity": "length", "value": "1"}),
    ];

    for value in cases {
        assert!(serde_json::from_value::<measurement::Length>(value).is_err());
    }
}

#[test]
fn test_measurement_wire_rejects_numeric_decimal_value() {
    assert!(
        serde_json::from_value::<measurement::Length>(json!({
            "quantity": "length",
            "value": 1,
            "unit": "m",
        }))
        .is_err(),
    );
}

#[test]
fn test_measurement_wire_rejects_unknown_unit_with_quantity_context() {
    let error = serde_json::from_value::<measurement::Length>(json!({
        "quantity": "length",
        "value": "1",
        "unit": "kg",
    }))
    .expect_err("unknown length unit should fail");

    assert!(error.to_string().contains("length"));
    assert!(error.to_string().contains("kg"));
}

#[test]
fn test_measurement_wire_ignores_additional_fields() {
    let value = serde_json::from_value::<measurement::Length>(json!({
        "quantity": "length",
        "value": "1",
        "unit": "m",
        "future": {"version": 2},
    }))
    .expect("additional fields should be ignored");

    assert_eq!(value.value.to_string(), "1");
}
```

注册 `tests/measure/internal/mod.rs` 和 `tests/measure/mod.rs` 的模块。

- [ ] **Step 2: 运行边界测试**

Run: `cargo +1.94.0 test --test mod measurement_wire -- --nocapture`

Expected: exit 0；未知单位错误文本包含 `unknown length unit: kg`。这里锁定现有契约，不修改生产代码或扩大可见性。

---

### Task 6: 补齐严格镜像测试目录

**Files:**
- Create: `tests/lib_tests.rs`
- Create: `tests/measurement_tests.rs`
- Create: `tests/unit_tests.rs`
- Move content: `tests/measure/private_tests.rs` -> `tests/private_tests.rs`
- Create: `tests/measure/mod_tests.rs`
- Create: `tests/measure/internal_tests.rs`
- Create: `tests/measure/units/internal_tests.rs`
- Create: `tests/measure/units/internal/mod.rs`
- Create: `tests/measure/units/internal/exact_torr_equivalent_tests.rs`
- Create: `tests/measurement/mod.rs`
- Create: `tests/measurement/support/mod.rs`
- Create: `tests/measurement/support/alias_assertions.rs`
- Create: `tests/measurement/*_tests.rs` for all 57 aliases
- Modify: `tests/mod.rs`
- Modify: `tests/measure/mod.rs`
- Modify: `tests/measure/units/mod.rs`

**Interfaces:**
- Produces: exact path pairing for all 132 current src Rust files。
- Preserves: existing behavior tests and public module paths。

- [ ] **Step 1: 建立别名共享断言宏**

`tests/measurement/support/alias_assertions.rs`：

```rust
// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared assertions for persisted measurement aliases.

macro_rules! assert_measurement_alias {
    ($alias:ident, $unit:ident) => {
        #[test]
        fn test_measurement_alias_uses_expected_unit_family() {
            use qubit_measure::{
                Decimal,
                Measurement,
                Unit,
                measurement,
                unit,
            };

            let selected_unit = *unit::$unit::all()
                .first()
                .expect("unit family should not be empty");
            let value: measurement::$alias =
                Measurement::new(Decimal::ONE, selected_unit);

            assert_eq!(value.quantity_name(), unit::$unit::QUANTITY);
        }
    };
}

pub(crate) use assert_measurement_alias;
```

`support/mod.rs` 只声明并重导出该宏。

- [ ] **Step 2: 创建 57 个别名镜像测试**

每个文件使用完整文件头、模块文档、导入共享宏，并包含一条明确调用。精确映射如下；除 Voltage 外 alias 与 unit 同名：

```text
acceleration=Acceleration
amount_of_substance=AmountOfSubstance
angle=Angle
angular_velocity=AngularVelocity
area=Area
capacitance=Capacitance
catalytic_activity=CatalyticActivity
catalytic_activity_concentration=CatalyticActivityConcentration
dynamic_viscosity=DynamicViscosity
electric_charge=ElectricCharge
electric_current=ElectricCurrent
electric_current_density=ElectricCurrentDensity
electric_field=ElectricField
electric_potential=ElectricPotential
electrical_conductance=ElectricalConductance
electrical_conductivity=ElectricalConductivity
electrical_resistance=ElectricalResistance
electrical_resistivity=ElectricalResistivity
energy=Energy
force=Force
frequency=Frequency
heat_capacity=HeatCapacity
heat_flux_density=HeatFluxDensity
illuminance=Illuminance
inductance=Inductance
kinematic_viscosity=KinematicViscosity
length=Length
luminance=Luminance
luminous_intensity=LuminousIntensity
magnetic_field_strength=MagneticFieldStrength
magnetic_flux=MagneticFlux
magnetic_flux_density=MagneticFluxDensity
mass=Mass
mass_concentration=MassConcentration
mass_density=MassDensity
mass_rate=MassRate
molality=Molality
molar_concentration=MolarConcentration
molar_mass=MolarMass
molar_volume=MolarVolume
power=Power
pressure=Pressure
radioactivity=Radioactivity
solid_angle=SolidAngle
specific_heat_capacity=SpecificHeatCapacity
specific_radioactivity=SpecificRadioactivity
surface_tension=SurfaceTension
temperature=Temperature
temperature_interval=TemperatureInterval
thermal_conductivity=ThermalConductivity
thermal_resistance=ThermalResistance
time=Time
torque=Torque
velocity=Velocity
voltage=Voltage,ElectricPotential
volume=Volume
volume_rate=VolumeRate
```

普通文件内容为：

```rust
use crate::measurement::support::assert_measurement_alias;

assert_measurement_alias!(Acceleration, Acceleration);
```

Voltage 文件使用 `assert_measurement_alias!(Voltage, ElectricPotential);`。`tests/measurement/mod.rs` 明确声明全部 57 个模块和 `mod support;`。

- [ ] **Step 3: 补齐根与 internal 镜像文件**

- `lib_tests.rs` 验证 `Measurement`, `Unit`, `measurement`, `unit` 根重导出可用。
- `measurement_tests.rs` 验证 `measurement::Length` 模块聚合路径。
- `unit_tests.rs` 验证 `unit::Length` 模块聚合路径。
- 把现有 private helper 测试内容移动到根 `private_tests.rs`，删除原路径模块声明。
- `measure/mod_tests.rs` 验证 measure 根公开核心类型路径。
- `measure/internal_tests.rs` 验证 Measurement Serde 通过 internal wire 往返。
- `measure/units/internal_tests.rs` 和 `measure/units/internal/exact_torr_equivalent_tests.rs` 通过 Pressure::MillimeterOfMercury 的公开 definition 验证 `101325 / 760`。

每个新增文件只包含与对应源码职责直接相关的一条或一组测试，不访问私有模块。

- [ ] **Step 4: 注册模块并运行别名测试**

在 `tests/mod.rs` 增加根测试模块和 `mod measurement;`；在两级 units/internal mod.rs 逐层声明。

Run:

```bash
cargo +1.94.0 test --test mod measurement:: -- --nocapture
cargo +1.94.0 test --test mod private_tests -- --nocapture
```

Expected: exit 0，57 个 alias 测试全部被发现。

- [ ] **Step 5: 审计精确路径配对**

Run:

```bash
ruby -e 'missing=Dir["src/**/*.rs"].reject{|f| File.exist?(f.sub(%r{^src/}, "tests/").sub(/\.rs$/, "_tests.rs"))}; abort missing.join("\n") unless missing.empty?'
```

Expected: exit 0 且无输出。

- [ ] **Step 6: 运行全部外部测试**

Run: `cargo +1.94.0 test --all-features --test mod -- --nocapture`

Expected: exit 0，无重复模块、重复测试名或未使用导入警告。

---

### Task 7: 修正生成代码 inline 分类

**Files:**
- Modify: `src/measure/units.rs`

**Interfaces:**
- No behavior or public API change。

- [ ] **Step 1: 修改两个属性**

给生成的 `FromStr::from_str` 添加 `#[inline(always)]`。把 `value_from_uom_approx` 的 `#[inline(always)]` 改为 `#[inline]`。

- [ ] **Step 2: 运行宏相关检查**

Run:

```bash
cargo +1.94.0 check --all-features
cargo +1.94.0 test --doc define_unit_family -- --nocapture
```

Expected: exit 0。

---

### Task 8: 完整验证与最终差异审计

**Files:**
- Verify all modified files。
- Modify only files changed by the prescribed formatter or in-scope failure fixes。

- [ ] **Step 1: 运行项目对齐脚本**

Run: `./align-ci.sh`

Expected: exit 0。脚本可能修改文件；立即运行 `git status --short` 和 `git --no-pager diff` 检查所有自动修改均在本计划范围。

- [ ] **Step 2: 运行完整 CI**

Run: `./ci-check.sh`

Expected: exit 0，包含 style、build、all-features tests、rustdoc、feature matrix、package、coverage threshold 和 audit，以及两个下游 fixture check。

- [ ] **Step 3: 条件覆盖率处理**

仅当 Step 2 明确报告 coverage 低于阈值时运行：

```bash
./coverage.sh json
```

Expected: 根据 JSON 缺口只补本轮业务分支测试，然后从 `./align-ci.sh`、`./ci-check.sh` 重跑。若 Step 2 覆盖率已达标，不单独运行该命令。

- [ ] **Step 4: 最终结构与范围审计**

Run:

```bash
ruby -e 'missing=Dir["src/**/*.rs"].reject{|f| File.exist?(f.sub(%r{^src/}, "tests/").sub(/\.rs$/, "_tests.rs"))}; abort missing.join("\n") unless missing.empty?'
git status --short
git --no-pager diff --stat
git --no-pager diff
```

Expected: 精确镜像无缺口；没有临时 uom 变异；没有 `.rs-ci` 修改；没有无关文件；所有改动均对应已确认的 7 项。

- [ ] **Step 5: 不提交并交付**

不执行 git add/commit/push。交付时逐项报告 RED-GREEN 证据、`./align-ci.sh` 和 `./ci-check.sh` 的实际 exit status、coverage 条件分支、测试文件新增/移动及剩余风险。
