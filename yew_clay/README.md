# YEW CLAY
This crate is a wrapper around the ClayUI components for React. This is a work in progress. Currently supported components are as follows:

* button

## Setup

Make sure you have the Clay stylesheet loaded in your project. For example, you can use the CDN as shown below.

```
<link
	rel="stylesheet"
	href="https://cdn.jsdelivr.net/npm/@clayui/css/lib/css/atlas.css"
/>
```

After that, use them as you would any other Yew component.

For items that require a spritemap, you'll need the URL to a Clay-compatible spritemap.
To get this from a cdn, use "https://cdn.jsdelivr.net/npm/@clayui/css/lib/images/icons/icons.svg";