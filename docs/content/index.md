# Wing

+ [Source](https://github.com/EthanJustice/wing)
+ [Docs](https://ethanjustice.github.io/wing)

Wing is a fairly simple static site generator, focusing on end performace.

## Commands

+ `serve` - serves a local version of the site and watches for changes in the project, triggering rebuilds when files are changed
+ `build` - builds a site
+ `new` - creates a new site

## Configuration

**Note**: this doesn't work right now, but is required as Wing will panic if it is not found (which will hopefully be changed in the future).

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

## Serve

The `serve` command will serve a local version of the site and watches for changes in the project, triggering rebuilds when files are changed.

**Note**: only `/static` and `/site` are served. `/site` is served from the root (`/`), meaning `/site/index.html` will be available on `localhost:8000/`, while `/static/index.css` will be available on `localhost:8000/static/index.css`.

## Templates

Wing uses [tera](https://tera.netlify.app/) for templating.

## Template Data

Wing comes with several built-in items that can be used within templates.

+ `title` (todo) - the name of the first top-level heading
+ `content` - HTML generated from the MarkDown file
+ `items` - a list of all items (as a list of paths)
+ `current` - the current item (as a path)
+ `frontmatter` - frontmatter from the template
  + `template` - template name
+ `created` - the (UTC) time the file was created
+ `modified` - the last (UTC) time the file was modified
