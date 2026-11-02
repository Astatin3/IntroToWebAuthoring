## IntroToWebAuthoring

This project is a custom webpage rendering project, build for a class.
It uses XML as the source of the structure, meant to resemble the layout XMLs of Android.

```xml
<ConstraintLayout>
    <!-- Sidebar -->
    <BoxView
        align_top_to_top="parent"
        align_left_to_left="parent"
        width="300"
        margin_left="10"
        height="parent"
    >
        <ColoredRectView r="10" g="20" b="40" />

        <VerticalLayout>

            <BoxView width="parent" height="45" margin_bottom="10">
                <ColoredRectView r="20" g="20" b="127" />
                <TextView
                    text="Bluee"
                    font_size="40."/>
            </BoxView>


            <BoxView width="parent" height="45" margin_bottom="10">
                <ColoredRectView r="127" g="127" b="127" />
                <TextView
                    text="Home"
                    font_size="40."/>
            </BoxView>

            <BoxView width="parent" height="45" margin_bottom="10">
                <ColoredRectView r="127" g="20" b="19" />
                <TextView
                    text="Red"
                    font_size="40."/>
            </BoxView>


            <BoxView width="parent" height="45" margin_bottom="10">
                <ColoredRectView r="20" g="127" b="21" />
                <TextView
                    text="Green"
                    font_size="40."/>
            </BoxView>
        </VerticalLayout>
    </BoxView>

    <!-- Main body -->
    <BoxView
        align_top_to_top="parent"
        align_bottom_to_bottom="parent"
        align_left_to_right="1"
        width="900"
        height="parent"
    >
            <ColoredRectView r="60" g="30" b="30" />

            <VerticalLayout>
                <BoxView width="parent" height="60" />

                <BoxView width="parent" height="30">
                    <TextView
                    text="Your text here"
                    font_size="30."
                /></BoxView>

            </VerticalLayout>
    </BoxView>

    <BoxView width="100" height="100"
        align_left_to_left="parent"
        align_top_to_top="parent"
        align_right_to_right="parent"
    >
        <TextView text="Hello!" font_size="60." />
    </BoxView>
</ConstraintLayout>
```


## Running:
```bash
# Install cargo (unix)
curl https://sh.rustup.rs -sSf | sh
# Install cargo (Windows, Chocolatey)
choco install rust
```
```bash
#Install trunk:
cargo install trunk
```
#### Compile and serve in debug mode:
```bash
trunk serve
```

#### Compile and serve in release mode:
```bash
trunk serve --release
```
