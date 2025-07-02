# ES|QL Reference

## Overview
ES|QL is Elasticsearch's piped query language using `|` operators to chain commands that transform data sequentially.

## Core Commands

### Data Source
```
FROM index_name | index_pattern* | index1,index2
```

### Filtering
```
WHERE field == "value"
WHERE @timestamp >= NOW() - 1h
WHERE field > 100 AND status != "error"
```

### Column Management
```
KEEP field1, field2          # Select columns
DROP field1, field2          # Remove columns  
RENAME old AS new           # Rename columns
```

### Aggregation
```
STATS count = COUNT(*) BY field
STATS avg_val = AVG(field), max_val = MAX(field) BY group_field
```

**Functions**: COUNT(), SUM(), AVG(), MIN(), MAX(), MEDIAN(), PERCENTILE()

### Sorting & Limiting
```
SORT field [ASC|DESC]
LIMIT 100
```

### Enrichment
```
ENRICH policy_name ON field
```

## Common Patterns

### Log Analysis
```
FROM logs-*
| WHERE @timestamp >= NOW() - 1h AND log.level == "ERROR"
| STATS count = COUNT(*) BY service.name
| SORT count DESC
| LIMIT 10
```

### Metrics Analysis
```
FROM metrics-*
| WHERE @timestamp >= NOW() - 1h
| STATS avg_cpu = AVG(cpu.percent) BY host.name
| WHERE avg_cpu > 80
| SORT avg_cpu DESC
```

### Time Series
```
FROM logs-*
| STATS count = COUNT(*) BY DATE_TRUNC("1h", @timestamp)
| SORT @timestamp
```

## Operators
- **Comparison**: `==`, `!=`, `>`, `>=`, `<`, `<=`
- **Logical**: `AND`, `OR`, `NOT`
- **Pattern**: `LIKE`, `RLIKE`
- **Null**: `IS NULL`, `IS NOT NULL`

## Best Practices
- Filter early with WHERE clauses
- Use specific time ranges
- Select only needed fields with KEEP
- Build queries incrementally

## Available Functions

These are callable function ES|QL, they have required and optional arguments as well as map that should appear in a named dictionary `{}` as the last argument.

- Bucket(field,buckets,from,to)
  - Required parameters:
    - field, types: ['integer', 'long', 'double', 'date', 'date_nanos'], description: Numeric or date expression from which to derive buckets.
    - buckets, types: ['integer', 'long', 'double', 'date_period', 'time_duration'], description: Target number of buckets, or desired bucket size if `from` and `to` parameters are omitted.
  - Optional parameters:
    - from, types: ['integer', 'long', 'double', 'date', 'keyword', 'text'], description: Start of the range. Can be a number, a date or a date expressed as a string.
    - to, types: ['integer', 'long', 'double', 'date', 'keyword', 'text'], description: End of the range. Can be a number, a date or a date expressed as a string.

- Bucket(field,buckets,from,to)
  - Required parameters:
    - field, types: ['integer', 'long', 'double', 'date', 'date_nanos'], description: Numeric or date expression from which to derive buckets.
    - buckets, types: ['integer', 'long', 'double', 'date_period', 'time_duration'], description: Target number of buckets, or desired bucket size if `from` and `to` parameters are omitted.
  - Optional parameters:
    - from, types: ['integer', 'long', 'double', 'date', 'keyword', 'text'], description: Start of the range. Can be a number, a date or a date expressed as a string.
    - to, types: ['integer', 'long', 'double', 'date', 'keyword', 'text'], description: End of the range. Can be a number, a date or a date expressed as a string.

- Categorize(field)
  - Required parameters:
    - field, types: ['text', 'keyword'], description: Expression to categorize

- Avg(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long'], description: Expression that outputs values to average.

- Count(field)
  - Required parameters:
  - Optional parameters:
    - field, types: ['aggregate_metric_double', 'boolean', 'cartesian_point', 'date', 'double', 'geo_point', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Expression that outputs values to be counted. If omitted, equivalent to `COUNT(*)` (the number of rows).

- CountDistinct(field,precision)
  - Required parameters:
    - field, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: Column or literal for which to count the number of distinct values.
  - Optional parameters:
    - precision, types: ['integer', 'long', 'unsigned_long'], description: Precision threshold. Refer to <<esql-agg-count-distinct-approximate>>. The maximum supported value is 40000. Thresholds above this number will have the same effect as a threshold of 40000. The default value is 3000.

- Max(field)
  - Required parameters:
    - field, types: ['aggregate_metric_double', 'boolean', 'double', 'integer', 'long', 'date', 'date_nanos', 'ip', 'keyword', 'text', 'long', 'version'], description: 

- Median(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long'], description: Expression that outputs values to calculate the median of.

- MedianAbsoluteDeviation(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long'], description: 

- Min(field)
  - Required parameters:
    - field, types: ['aggregate_metric_double', 'boolean', 'double', 'integer', 'long', 'date', 'date_nanos', 'ip', 'keyword', 'text', 'long', 'version'], description: 

- Percentile(number,percentile)
  - Required parameters:
    - number, types: ['double', 'integer', 'long'], description: 
    - percentile, types: ['double', 'integer', 'long'], description: 

- Sample(field,limit)
  - Required parameters:
    - field, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: The field to collect sample values for.
    - limit, types: ['integer'], description: The maximum number of values to collect.

- StdDev(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long'], description: 

- Sum(number)
  - Required parameters:
    - number, types: ['aggregate_metric_double', 'double', 'integer', 'long'], description: 

- Top(field,limit,order)
  - Required parameters:
    - field, types: ['boolean', 'double', 'integer', 'long', 'date', 'ip', 'keyword', 'text'], description: The field to collect the top values for.
    - limit, types: ['integer'], description: The maximum number of values to collect.
    - order, types: ['keyword'], description: The order to calculate the top values. Either `asc` or `desc`.

- Values(field)
  - Required parameters:
    - field, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: 

- WeightedAvg(number,weight)
  - Required parameters:
    - number, types: ['double', 'integer', 'long'], description: A numeric value.
    - weight, types: ['double', 'integer', 'long'], description: A numeric weight.

- Abs(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Acos(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Number between -1 and 1. If `null`, the function returns `null`.

- Asin(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Number between -1 and 1. If `null`, the function returns `null`.

- Atan(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Atan2(y_coordinate,x_coordinate)
  - Required parameters:
    - y_coordinate, types: ['double', 'integer', 'long', 'unsigned_long'], description: y coordinate. If `null`, the function returns `null`.
    - x_coordinate, types: ['double', 'integer', 'long', 'unsigned_long'], description: x coordinate. If `null`, the function returns `null`.

- Cbrt(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Ceil(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- CopySign(magnitude,sign)
  - Required parameters:
    - magnitude, types: ['double', 'float', 'integer', 'long'], description: The expression providing the magnitude of the result. Must be a numeric type.
    - sign, types: ['double', 'float', 'integer', 'long'], description: The expression providing the sign of the result. Must be a numeric type.

- Cos(angle)
  - Required parameters:
    - angle, types: ['double', 'integer', 'long', 'unsigned_long'], description: An angle, in radians. If `null`, the function returns `null`.

- Cosh(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- E()

- Exp(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Floor(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Greatest(first,rest)
  - Required parameters:
    - first, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: First of the columns to evaluate.
  - Optional parameters:
    - rest, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: The rest of the columns to evaluate.

- Hypot(number1,number2)
  - Required parameters:
    - number1, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.
    - number2, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Least(first,rest)
  - Required parameters:
    - first, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: First of the columns to evaluate.
  - Optional parameters:
    - rest, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: The rest of the columns to evaluate.

- Log(base,number)
  - Required parameters:
    - number, types: ['integer', 'unsigned_long', 'long', 'double'], description: Numeric expression. If `null`, the function returns `null`.
  - Optional parameters:
    - base, types: ['integer', 'unsigned_long', 'long', 'double'], description: Base of logarithm. If `null`, the function returns `null`. If not provided, this function returns the natural logarithm (base e) of a value.

- Log10(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Pi()

- Pow(base,exponent)
  - Required parameters:
    - base, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression for the base. If `null`, the function returns `null`.
    - exponent, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression for the exponent. If `null`, the function returns `null`.

- Round(number,decimals)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: The numeric value to round. If `null`, the function returns `null`.
  - Optional parameters:
    - decimals, types: ['integer', 'long'], description: The number of decimal places to round to. Defaults to 0. If `null`, the function returns `null`.

- RoundTo(field,points)
  - Required parameters:
    - field, types: ['double', 'integer', 'long', 'date', 'date_nanos'], description: The numeric value to round. If `null`, the function returns `null`.
    - points, types: ['double', 'integer', 'long', 'date', 'date_nanos'], description: Remaining rounding points. Must be constants.

- Scalb(d,scaleFactor)
  - Required parameters:
    - d, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression for the multiplier. If `null`, the function returns `null`.
    - scaleFactor, types: ['integer', 'long'], description: Numeric expression for the scale factor. If `null`, the function returns `null`.

- Signum(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Sin(angle)
  - Required parameters:
    - angle, types: ['double', 'integer', 'long', 'unsigned_long'], description: An angle, in radians. If `null`, the function returns `null`.

- Sinh(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Sqrt(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Tan(angle)
  - Required parameters:
    - angle, types: ['double', 'integer', 'long', 'unsigned_long'], description: An angle, in radians. If `null`, the function returns `null`.

- Tanh(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Numeric expression. If `null`, the function returns `null`.

- Tau()

- BitLength(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- ByteLength(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- Concat(string1,string2)
  - Required parameters:
    - string1, types: ['keyword', 'text'], description: Strings to concatenate.
    - string2, types: ['keyword', 'text'], description: Strings to concatenate.

- EndsWith(str,suffix)
  - Required parameters:
    - str, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.
    - suffix, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- Hash(algorithm,input)
  - Required parameters:
    - algorithm, types: ['keyword', 'text'], description: Hash algorithm to use.
    - input, types: ['keyword', 'text'], description: Input to hash.

- Left(string,length)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: The string from which to return a substring.
    - length, types: ['integer'], description: The number of characters to return.

- Length(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- Locate(string,substring,start)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: An input string
    - substring, types: ['keyword', 'text'], description: A substring to locate in the input string
  - Optional parameters:
    - start, types: ['integer'], description: The start index

- LTrim(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- Md5(input)
  - Required parameters:
    - input, types: ['keyword', 'text'], description: Input to hash.

- Repeat(string,number)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression.
    - number, types: ['integer'], description: Number times to repeat.

- Replace(string,regex,newString)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression.
    - regex, types: ['keyword', 'text'], description: Regular expression.
    - newString, types: ['keyword', 'text'], description: Replacement string.

- Reverse(str)
  - Required parameters:
    - str, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- Right(string,length)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: The string from which to returns a substring.
    - length, types: ['integer'], description: The number of characters to return.

- RTrim(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- Sha1(input)
  - Required parameters:
    - input, types: ['keyword', 'text'], description: Input to hash.

- Sha256(input)
  - Required parameters:
    - input, types: ['keyword', 'text'], description: Input to hash.

- Space(number)
  - Required parameters:
    - number, types: ['integer'], description: Number of spaces in result.

- StartsWith(str,prefix)
  - Required parameters:
    - str, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.
    - prefix, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- Substring(string,start,length)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.
    - start, types: ['integer'], description: Start position.
  - Optional parameters:
    - length, types: ['integer'], description: Length of the substring from the start position. Optional; if omitted, all positions after `start` are returned.

- ToLower(str)
  - Required parameters:
    - str, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.
The input can be a single- or multi-valued column or an expression.

- ToUpper(str)
  - Required parameters:
    - str, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.
The input can be a single- or multi-valued column or an expression.

- Trim(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.

- DateDiff(unit,startTimestamp,endTimestamp)
  - Required parameters:
    - unit, types: ['keyword', 'text'], description: Time difference unit
    - startTimestamp, types: ['date', 'date_nanos'], description: A string representing a start timestamp
    - endTimestamp, types: ['date', 'date_nanos'], description: A string representing an end timestamp

- DateExtract(datePart,date)
  - Required parameters:
    - datePart, types: ['keyword', 'text'], description: Part of the date to extract.

Can be: `aligned_day_of_week_in_month`, `aligned_day_of_week_in_year`, `aligned_week_of_month`, `aligned_week_of_year`,
`ampm_of_day`, `clock_hour_of_ampm`, `clock_hour_of_day`, `day_of_month`, `day_of_week`, `day_of_year`, `epoch_day`,
`era`, `hour_of_ampm`, `hour_of_day`, `instant_seconds`, `micro_of_day`, `micro_of_second`, `milli_of_day`,
`milli_of_second`, `minute_of_day`, `minute_of_hour`, `month_of_year`, `nano_of_day`, `nano_of_second`,
`offset_seconds`, `proleptic_month`, `second_of_day`, `second_of_minute`, `year`, or `year_of_era`.
Refer to {javadoc8}/java/time/temporal/ChronoField.html[java.time.temporal.ChronoField]
for a description of these values.

If `null`, the function returns `null`.
    - date, types: ['date', 'date_nanos'], description: Date expression. If `null`, the function returns `null`.

- DateFormat(dateFormat,date)
  - Required parameters:
    - date, types: ['date', 'date_nanos'], description: Date expression. If `null`, the function returns `null`.
  - Optional parameters:
    - dateFormat, types: ['keyword', 'text', 'date', 'date_nanos'], description: Date format (optional).  If no format is specified, the `yyyy-MM-dd'T'HH:mm:ss.SSSZ` format is used.
If `null`, the function returns `null`.

- DateParse(datePattern,dateString)
  - Required parameters:
    - dateString, types: ['keyword', 'text'], description: Date expression as a string. If `null` or an empty string, the function returns `null`.
  - Optional parameters:
    - datePattern, types: ['keyword', 'text'], description: The date format. Refer to the
{javadoc14}/java.base/java/time/format/DateTimeFormatter.html[`DateTimeFormatter` documentation] for the syntax.
If `null`, the function returns `null`.

- DateTrunc(interval,date)
  - Required parameters:
    - interval, types: ['date_period', 'time_duration'], description: Interval; expressed using the timespan literal syntax.
    - date, types: ['date', 'date_nanos'], description: Date expression

- Now()

- SpatialCentroid(field)
  - Required parameters:
    - field, types: ['geo_point', 'cartesian_point'], description: 

- SpatialContains(geomA,geomB)
  - Required parameters:
    - geomA, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
    - geomB, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
The second parameter must also have the same coordinate system as the first.
This means it is not possible to combine `geo_*` and `cartesian_*` parameters.

- SpatialDisjoint(geomA,geomB)
  - Required parameters:
    - geomA, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
    - geomB, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
The second parameter must also have the same coordinate system as the first.
This means it is not possible to combine `geo_*` and `cartesian_*` parameters.

- StDistance(geomA,geomB)
  - Required parameters:
    - geomA, types: ['geo_point', 'cartesian_point'], description: Expression of type `geo_point` or `cartesian_point`.
If `null`, the function returns `null`.
    - geomB, types: ['geo_point', 'cartesian_point'], description: Expression of type `geo_point` or `cartesian_point`.
If `null`, the function returns `null`.
The second parameter must also have the same coordinate system as the first.
This means it is not possible to combine `geo_point` and `cartesian_point` parameters.

- StEnvelope(geometry)
  - Required parameters:
    - geometry, types: ['geo_point', 'geo_shape', 'cartesian_point', 'cartesian_shape'], description: Expression of type `geo_point`, `geo_shape`, `cartesian_point` or `cartesian_shape`. If `null`, the function returns `null`.

- SpatialExtent(field)
  - Required parameters:
    - field, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: 

- StGeohash(geometry,precision,bounds)
  - Required parameters:
    - geometry, types: ['geo_point'], description: Expression of type `geo_point`. If `null`, the function returns `null`.
    - precision, types: ['integer'], description: Expression of type `integer`. If `null`, the function returns `null`.
Valid values are between [1 and 12](https://en.wikipedia.org/wiki/Geohash).
  - Optional parameters:
    - bounds, types: ['geo_shape'], description: Optional bounds to filter the grid tiles, a `geo_shape` of type `BBOX`.
Use [`ST_ENVELOPE`](#esql-st_envelope) if the `geo_shape` is of any other type.

- StGeohashToLong(grid_id)
  - Required parameters:
    - grid_id, types: ['keyword', 'long'], description: Input geohash grid-id. The input can be a single- or multi-valued column or an expression.

- StGeohashToString(grid_id)
  - Required parameters:
    - grid_id, types: ['keyword', 'long'], description: Input geohash grid-id. The input can be a single- or multi-valued column or an expression.

- StGeohex(geometry,precision,bounds)
  - Required parameters:
    - geometry, types: ['geo_point'], description: Expression of type `geo_point`. If `null`, the function returns `null`.
    - precision, types: ['integer'], description: Expression of type `integer`. If `null`, the function returns `null`.
Valid values are between [0 and 15](https://h3geo.org/docs/core-library/restable/).
  - Optional parameters:
    - bounds, types: ['geo_shape'], description: Optional bounds to filter the grid tiles, a `geo_shape` of type `BBOX`.
Use [`ST_ENVELOPE`](#esql-st_envelope) if the `geo_shape` is of any other type.

- StGeohexToLong(grid_id)
  - Required parameters:
    - grid_id, types: ['keyword', 'long'], description: Input geohex grid-id. The input can be a single- or multi-valued column or an expression.

- StGeohexToString(grid_id)
  - Required parameters:
    - grid_id, types: ['keyword', 'long'], description: Input Geohex grid-id. The input can be a single- or multi-valued column or an expression.

- StGeotile(geometry,precision,bounds)
  - Required parameters:
    - geometry, types: ['geo_point'], description: Expression of type `geo_point`. If `null`, the function returns `null`.
    - precision, types: ['integer'], description: Expression of type `integer`. If `null`, the function returns `null`.
Valid values are between [0 and 29](https://wiki.openstreetmap.org/wiki/Zoom_levels).
  - Optional parameters:
    - bounds, types: ['geo_shape'], description: Optional bounds to filter the grid tiles, a `geo_shape` of type `BBOX`.
Use [`ST_ENVELOPE`](#esql-st_envelope) if the `geo_shape` is of any other type.

- StGeotileToLong(grid_id)
  - Required parameters:
    - grid_id, types: ['keyword', 'long'], description: Input geotile grid-id. The input can be a single- or multi-valued column or an expression.

- StGeotileToString(grid_id)
  - Required parameters:
    - grid_id, types: ['keyword', 'long'], description: Input geotile grid-id. The input can be a single- or multi-valued column or an expression.

- SpatialIntersects(geomA,geomB)
  - Required parameters:
    - geomA, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
    - geomB, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
The second parameter must also have the same coordinate system as the first.
This means it is not possible to combine `geo_*` and `cartesian_*` parameters.

- SpatialWithin(geomA,geomB)
  - Required parameters:
    - geomA, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
    - geomB, types: ['geo_point', 'cartesian_point', 'geo_shape', 'cartesian_shape'], description: Expression of type `geo_point`, `cartesian_point`, `geo_shape` or `cartesian_shape`.
If `null`, the function returns `null`.
The second parameter must also have the same coordinate system as the first.
This means it is not possible to combine `geo_*` and `cartesian_*` parameters.

- StX(point)
  - Required parameters:
    - point, types: ['geo_point', 'cartesian_point'], description: Expression of type `geo_point` or `cartesian_point`. If `null`, the function returns `null`.

- StXMax(point)
  - Required parameters:
    - point, types: ['geo_point', 'geo_shape', 'cartesian_point', 'cartesian_shape'], description: Expression of type `geo_point`, `geo_shape`, `cartesian_point` or `cartesian_shape`. If `null`, the function returns `null`.

- StXMin(point)
  - Required parameters:
    - point, types: ['geo_point', 'geo_shape', 'cartesian_point', 'cartesian_shape'], description: Expression of type `geo_point`, `geo_shape`, `cartesian_point` or `cartesian_shape`. If `null`, the function returns `null`.

- StY(point)
  - Required parameters:
    - point, types: ['geo_point', 'cartesian_point'], description: Expression of type `geo_point` or `cartesian_point`. If `null`, the function returns `null`.

- StYMax(point)
  - Required parameters:
    - point, types: ['geo_point', 'geo_shape', 'cartesian_point', 'cartesian_shape'], description: Expression of type `geo_point`, `geo_shape`, `cartesian_point` or `cartesian_shape`. If `null`, the function returns `null`.

- StYMin(point)
  - Required parameters:
    - point, types: ['geo_point', 'geo_shape', 'cartesian_point', 'cartesian_shape'], description: Expression of type `geo_point`, `geo_shape`, `cartesian_point` or `cartesian_shape`. If `null`, the function returns `null`.

- Case(condition,trueValue)
  - Required parameters:
    - condition, types: ['boolean'], description: A condition.
    - trueValue, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: The value that’s returned when the corresponding condition is the first to evaluate to `true`. The default value is returned when no condition matches.

- Coalesce(first,rest)
  - Required parameters:
    - first, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date_nanos', 'date', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: Expression to evaluate.
  - Optional parameters:
    - rest, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date_nanos', 'date', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: Other expression to evaluate.

- CIDRMatch(ip,blockX)
  - Required parameters:
    - ip, types: ['ip'], description: IP address of type `ip` (both IPv4 and IPv6 are supported).
    - blockX, types: ['keyword', 'text'], description: CIDR block to test the IP against.

- IpPrefix(ip,prefixLengthV4,prefixLengthV6)
  - Required parameters:
    - ip, types: ['ip'], description: IP address of type `ip` (both IPv4 and IPv6 are supported).
    - prefixLengthV4, types: ['integer'], description: Prefix length for IPv4 addresses.
    - prefixLengthV6, types: ['integer'], description: Prefix length for IPv6 addresses.

- FromBase64(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: A base64 string.

- ToAggregateMetricDouble(number)
  - Required parameters:
    - number, types: ['double', 'long', 'unsigned_long', 'integer', 'aggregate_metric_double'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToAggregateMetricDouble(number)
  - Required parameters:
    - number, types: ['double', 'long', 'unsigned_long', 'integer', 'aggregate_metric_double'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToBase64(string)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: A string.

- ToBoolean(field)
  - Required parameters:
    - field, types: ['boolean', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToBoolean(field)
  - Required parameters:
    - field, types: ['boolean', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToCartesianPoint(field)
  - Required parameters:
    - field, types: ['cartesian_point', 'keyword', 'text'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToCartesianShape(field)
  - Required parameters:
    - field, types: ['cartesian_point', 'cartesian_shape', 'keyword', 'text'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToDateNanos(field)
  - Required parameters:
    - field, types: ['date', 'date_nanos', 'keyword', 'text', 'double', 'long', 'unsigned_long'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToDateNanos(field)
  - Required parameters:
    - field, types: ['date', 'date_nanos', 'keyword', 'text', 'double', 'long', 'unsigned_long'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToDatePeriod(field)
  - Required parameters:
    - field, types: ['date_period', 'keyword', 'text'], description: Input value. The input is a valid constant date period expression.

- ToDatetime(field)
  - Required parameters:
    - field, types: ['date', 'date_nanos', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToDouble(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer', 'counter_double', 'counter_integer', 'counter_long'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToDegrees(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToDouble(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer', 'counter_double', 'counter_integer', 'counter_long'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToDatetime(field)
  - Required parameters:
    - field, types: ['date', 'date_nanos', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToGeoPoint(field)
  - Required parameters:
    - field, types: ['geo_point', 'keyword', 'text'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToGeoShape(field)
  - Required parameters:
    - field, types: ['geo_point', 'geo_shape', 'keyword', 'text'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToInteger(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer', 'counter_integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToInteger(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer', 'counter_integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToIp(field)
  - Required parameters:
    - field, types: ['ip', 'keyword', 'text'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToLong(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'date_nanos', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer', 'counter_integer', 'counter_long'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToRadians(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToString(field)
  - Required parameters:
    - field, types: ['aggregate_metric_double', 'boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToString(field)
  - Required parameters:
    - field, types: ['aggregate_metric_double', 'boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToTimeDuration(field)
  - Required parameters:
    - field, types: ['time_duration', 'keyword', 'text'], description: Input value. The input is a valid constant time duration expression.

- ToUnsignedLong(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToUnsignedLong(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToUnsignedLong(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'keyword', 'text', 'double', 'long', 'unsigned_long', 'integer'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToVersion(field)
  - Required parameters:
    - field, types: ['keyword', 'text', 'version'], description: Input value. The input can be a single- or multi-valued column or an expression.

- ToVersion(field)
  - Required parameters:
    - field, types: ['keyword', 'text', 'version'], description: Input value. The input can be a single- or multi-valued column or an expression.

- MvAppend(field1,field2)
  - Required parameters:
    - field1, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: 
    - field2, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: 

- MvAvg(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Multivalue expression.

- MvConcat(string,delim)
  - Required parameters:
    - string, types: ['text', 'keyword'], description: Multivalue expression.
    - delim, types: ['text', 'keyword'], description: Delimiter.

- MvCount(field)
  - Required parameters:
    - field, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Multivalue expression.

- MvDedupe(field)
  - Required parameters:
    - field, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Multivalue expression.

- MvFirst(field)
  - Required parameters:
    - field, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Multivalue expression.

- MvLast(field)
  - Required parameters:
    - field, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Multivalue expression.

- MvMax(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Multivalue expression.

- MvMedian(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Multivalue expression.

- MvMedianAbsoluteDeviation(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Multivalue expression.

- MvMin(field)
  - Required parameters:
    - field, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Multivalue expression.

- MvPercentile(number,percentile)
  - Required parameters:
    - number, types: ['double', 'integer', 'long'], description: Multivalue expression.
    - percentile, types: ['double', 'integer', 'long'], description: The percentile to calculate. Must be a number between 0 and 100. Numbers out of range will return a null instead.

- MvPSeriesWeightedSum(number,p)
  - Required parameters:
    - number, types: ['double'], description: Multivalue expression.
    - p, types: ['double'], description: It is a constant number that represents the *p* parameter in the P-Series. It impacts every element’s contribution to the weighted sum.

- MvSlice(field,start,end)
  - Required parameters:
    - field, types: ['boolean', 'cartesian_point', 'cartesian_shape', 'date', 'date_nanos', 'double', 'geo_point', 'geo_shape', 'integer', 'ip', 'keyword', 'long', 'text', 'unsigned_long', 'version'], description: Multivalue expression. If `null`, the function returns `null`.
    - start, types: ['integer'], description: Start position. If `null`, the function returns `null`. The start argument can be negative. An index of -1 is used to specify the last value in the list.
  - Optional parameters:
    - end, types: ['integer'], description: End position(included). Optional; if omitted, the position at `start` is returned. The end argument can be negative. An index of -1 is used to specify the last value in the list.

- MvSort(field,order)
  - Required parameters:
    - field, types: ['boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'keyword', 'long', 'text', 'version'], description: Multivalue expression. If `null`, the function returns `null`.
  - Optional parameters:
    - order, types: ['keyword'], description: Sort order. The valid options are ASC and DESC, the default is ASC.

- MvSum(number)
  - Required parameters:
    - number, types: ['double', 'integer', 'long', 'unsigned_long'], description: Multivalue expression.

- MvZip(string1,string2,delim)
  - Required parameters:
    - string1, types: ['keyword', 'text'], description: Multivalue expression.
    - string2, types: ['keyword', 'text'], description: Multivalue expression.
  - Optional parameters:
    - delim, types: ['keyword', 'text'], description: Delimiter. Optional; if omitted, `,` is used as a default delimiter.

- Split(string,delim)
  - Required parameters:
    - string, types: ['keyword', 'text'], description: String expression. If `null`, the function returns `null`.
    - delim, types: ['keyword', 'text'], description: Delimiter. Only single byte delimiters are currently supported.

- Kql(query)
  - Required parameters:
    - query, types: ['keyword', 'text'], description: Query string in KQL query string format.

- Match(field,query)
  - Required parameters:
    - field, types: ['keyword', 'text', 'boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'long', 'unsigned_long', 'version'], description: Field that the query will target.
    - query, types: ['keyword', 'boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'long', 'unsigned_long', 'version'], description: Value to find in the provided field.

- MultiMatch(query,fields)
  - Required parameters:
    - query, types: ['keyword', 'boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'long', 'text', 'unsigned_long', 'version'], description: Value to find in the provided fields.
    - fields, types: ['keyword', 'boolean', 'date', 'date_nanos', 'double', 'integer', 'ip', 'long', 'unsigned_long', 'version'], description: Fields to use for matching

- QueryString(query)
  - Required parameters:
    - query, types: ['keyword', 'text'], description: Query string in Lucene query string format.
