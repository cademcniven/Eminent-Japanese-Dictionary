# <p align="center">EJDIC</p>
$${\color{lightgreen}E\color{white}minent \space \color{lightgreen}J\color{white}apanese \space \color{lightgreen}DIC\color{white}tionary \space format}$$

## WIP DOESNT DO MUCH RN

jpdic is [XML](https://www.w3schools.com/xml/xml_whatis.asp), loosely based on the [ODict](https://github.com/TheOpenDictionary/odict) format, but completely specialized for Japanese.

## Components

A dictionary file is built with the following components:

### Dictionary

The `dictionary` tag is the outermost tag. Every dictionary file will contain exactly 1 of these. It is required for a valid dictionary.

#### Attributes

* name
	* the name of the dictionary
	* **required**

#### Children

* [entry](#entry)

#### Examples

```xml
<dictionary name="Example Dictionary">
<!-- like 500k entries here -->
</dictionary>
```

### Entry

The `entry` tag defines a single dictionary entry. The entry may have multiple usages. Each entry has exactly 1 reading, so a new entry will have to be made if there are multiple readings.

#### Attributes

* term
	* the word that the entry is about
	* **required**
* reading
	* the kana-only version of `term`

#### Children

* [see](#see)
* [frequency](#frequency)
* [alt](#alt)
* [usage](#usage)

#### Examples

```xml
<entry term="見える" reading="まみえる">
	<frequency>260</frequency>
	<alt>謁える</alt>
	<usage>
		<tag>v1</tag>
		<tag>vi</tag>
		<pitch>3</pitch>
		<definition value="〘hum〙 to have an audience; to meet; to see.​" />
		<definition value="to face (an enemy); to confront.​" />
		<definition value="to serve (esp. as one's wife)." />
	</usage>
</entry>
```

### See

The `see` tag contains another related term.

#### Examples

```xml
<see>見る</see>
```

### Frequency

The `frequency` tag contains information about how common a word is according to some [frequency list](https://en.wikipedia.org/wiki/Word_list).

#### Examples

```xml
<frequency>260</frequency>
```

### Alt

The `alt` tag describes alternate kanji forms of the word.

#### Examples

```xml
<alt>謁える</alt>
```

### Usage

`usage` is equivalent to a "sense" in some other dictionary formats. 

#### Children

* [tag](#tag)
* [pitch](#pitch)
* [definition](#definition)

#### Examples

```xml
<usage>
	<tag>n</tag>
	<pitch>1</pitch>
	<definition value="eye; eyeball​">
		<example value="彼女はきれいな目をしている。" translation="She has beautiful eyes" />
	</definition>
	<definition value="eyesight; sight; vision">
		<see>目が見える</see>
		<example value="ヘレン・ケラーは目と耳と口が不自由だった。"
			translation="Helen Keller was blind, deaf and mute." />
	</definition>
	<definition value="look; stare; gaze; glance​">
		<example value="彼は崇拝の眼で彼女を眺めた。"
			translation="He regarded her with worship in his eyes." />
	</definition>
	<definition
		value="notice; attention; observation; eyes (of the world, public, etc.)​" />
	<definition value="an experience​" />
	<definition value="viewpoint​" />
	<definition value="discrimination; discernment; judgement; eye (e.g. for quality)​" />
	<definition value="(an) appearance​" />
	<definition value="chance (of success); possibility (of a good result)​">
		<see>目がない</see>
	</definition>
	<definition
		value="spacing (between crossed strands of a net, mesh, etc.); opening; stitch; texture; weave">
		<see>編み目</see>
	</definition>
	<definition value="grain (of wood)​" />
	<definition value="eye (of a storm, needle, etc.)​" />
	<definition value="intersection (on a go board); square (on a chessboard)​" />
	<definition value="dot (on a dice); pip; rolled number​" />
	<definition value="graduation; division (of a scale)​" />
	<definition value="tooth (of a saw, comb, etc.)​" />
</usage>
```

```xml
<usage>
	<tag>suf</tag>
	<pitch>1</pitch>
	<definition value="somewhat; -ish">
		<see>大きめ</see>
		<info>after adjective stem</info>
	</definition>
	<definition value="point (e.g. of change)">
		<see>変わり目</see>
		<see>折れ目</see>
		<info>after -masu stem of verb</info>
	</definition>
</usage>
```

### Tag

The `tag` tag is used for marking a usage as a certain kind of word (noun, verb, yoji, fish, etc.). 

Valid tags are the ones allowed in JMDict. See [Jisho Docs](https://jisho.org/docs) for a full list.

#### Examples

```xml
<tag>v1</tag>
<tag>vi</tag>
```

### Pitch

The `pitch` tag contains pitch accent information for the usage of a word.

#### Attributes

* value
	* the [pitch number](https://en.wikibooks.org/wiki/Japanese/Pitch_accent)
	* **required**

#### Examples

```xml
<pitch>1</pitch>
```

### Definition

The `definition` tag contains the definition of the usage, and optionally example sentences.

#### Attributes

* value
	* the definition
	* **required**

#### Children

* [see](#see)
* [example](#example)

#### Examples

```xml
<definition value="grain (of wood)​" />
```

```xml
<definition value="chance (of success); possibility (of a good result)​">
	<see>目がない</see>
</definition>
```

```xml
<definition value="eye; eyeball​">
	<example value="彼女はきれいな目をしている。" translation="She has beautiful eyes" />
</definition>
```

```xml
<definition value="eyesight; sight; vision">
	<see>目が見える</see>
	<example value="ヘレン・ケラーは目と耳と口が不自由だった。"
		translation="Helen Keller was blind, deaf and mute." />
</definition>
```
### Example

The `example` tag contains an example sentence, and optionally a translation of the example sentence.

#### Attributes

* value
	* the example sentence
	* **required**
* translation
	* a translation of the example sentence

#### Examples

```xml
<example value="１１ページの地図はとても奇妙に見える。逆さまにしてみると、見慣れた地図になる。" translation="The map on page 11 looks very strange. Turn it upside down. Then it becomes a familiar map to you." />
```

```xml
<example value="彼女はきれいな目をしている。" />
```

## Full Example

Here's a larger example of a dictionary with several entries to give an idea of how entries with different kinds of data might be structured.

```xml
<dictionary name="Test Dictionary">
    <entry term="見える" reading="みえる">
        <see>見る</see>
        <frequency>260</frequency>
        <usage>
            <tag>v1</tag>
            <tag>vi</tag>
            <pitch>2</pitch>
            <definition value="to be seen; to be visible; to be in sight​">
                <example value="聖霊がある方の上に下って、その上にとどまられるのが見えたなら、その方こそ聖霊によってバプテスマを授ける方である。"
                    translation="The man on whom you see the Spirit come down and remain is he who will baptize with the Holy Spirit." />
            </definition>
            <definition value="to look; to seem; to appear​">
                <example value="１１ページの地図はとても奇妙に見える。逆さまにしてみると、見慣れた地図になる。"
                    translation="The map on page 11 looks very strange. Turn it upside down. Then it becomes a familiar map to you." />
            </definition>
            <definition value="to come" />
        </usage>
    </entry>
    <entry term="見える" reading="まみえる">
        <frequency>260</frequency>
        <alt>謁える</alt>
        <usage>
            <tag>v1</tag>
            <tag>vi</tag>
            <pitch>3</pitch>
            <definition value="〘hum〙 to have an audience; to meet; to see.​" />
            <definition value="to face (an enemy); to confront.​" />
            <definition value="to serve (esp. as one's wife)." />
        </usage>
    </entry>
    <entry term="目" reading="め">
        <frequency>177</frequency>
        <alt>眼</alt>
        <usage>
            <tag>n</tag>
            <pitch>1</pitch>
            <definition value="eye; eyeball​">
                <example value="彼女はきれいな目をしている。" translation="She has beautiful eyes" />
            </definition>
            <definition value="eyesight; sight; vision">
                <see>目が見える</see>
                <example value="ヘレン・ケラーは目と耳と口が不自由だった。"
                    translation="Helen Keller was blind, deaf and mute." />
            </definition>
            <definition value="look; stare; gaze; glance​">
                <example value="彼は崇拝の眼で彼女を眺めた。"
                    translation="He regarded her with worship in his eyes." />
            </definition>
            <definition
                value="notice; attention; observation; eyes (of the world, public, etc.)​" />
            <definition value="an experience​" />
            <definition value="viewpoint​" />
            <definition value="discrimination; discernment; judgement; eye (e.g. for quality)​" />
            <definition value="(an) appearance​" />
            <definition value="chance (of success); possibility (of a good result)​">
                <see>目がない</see>
            </definition>
            <definition
                value="spacing (between crossed strands of a net, mesh, etc.); opening; stitch; texture; weave">
                <see>編み目</see>
            </definition>
            <definition value="grain (of wood)​" />
            <definition value="eye (of a storm, needle, etc.)​" />
            <definition value="intersection (on a go board); square (on a chessboard)​" />
            <definition value="dot (on a dice); pip; rolled number​" />
            <definition value="graduation; division (of a scale)​" />
            <definition value="tooth (of a saw, comb, etc.)​" />
        </usage>
        <usage>
            <tag>suf</tag>
            <pitch>1</pitch>
            <definition value="somewhat; -ish">
                <see>大きめ</see>
                <info>after adjective stem</info>
            </definition>
            <definition value="point (e.g. of change)">
                <see>変わり目</see>
                <see>折れ目</see>
                <info>after -masu stem of verb</info>
            </definition>
        </usage>
    </entry>
</dictionary>
```
