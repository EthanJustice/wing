# Wing

+ [Source](https://github.com/EthanJustice/wing)
+ [Docs](https://ethanjustice.github.io/wing)

Wing is a fairly simple static site generator, focusing on end performace.

## Config

Wing can be configured by placing a `.wing` file in your site's root directory.  It's written in JSON.

Wing's default configuration file looks like this:

```json
{
    "rss": false,
    "siteMap": false,
    "linkType": "relative",
    "optimisationLevel": "none",
    "preScripts": [],
    "postScripts": []
}
```

## Templates

Wing uses [tera](https://tera.netlify.app/) for templating.

## Built-ins

Wing comes with several built-in items.

+ `content` - HTML generated from the MarkDown file
+ `items` - a list of all items (as paths)
+ `current` - the current item (as a path)
+ `frontmatter` - frontmatter from the template
  + `template` - template name
