from flask import Flask, render_template, redirect

app = Flask(__name__)

articles = [
    {
        "id": 342,
        "title": "Hello",
        "small_body": "This is the articles short desc",
        "friendly_date": "5 hours ago",
        "is_battle": True,
        "war_num": 81,
    },
    {
        "id": 12,
        "title": "Another",
        "friendly_date": "5 hours ago",
        "is_battle": True,
        "war_num": 81,
    },
    {
        "id": 234,
        "title": "This is all in templating",
        "small_body": "This is another articles short desc",
        "friendly_date": "In like 2 years",
        "is_battle": True,
        "war_num": 88,
    },
    {
        "title": "WAR BREAKS OUT",
        "small_body": "During the winter blizzards, a new storm is starting to ravage the heartlands of aristotka, but this is no storm. This is war.",
        "friendly_date": "In like 2 years",
        "is_battle": False,
        "war_num": 88,
    },
]


@app.route("/base")
def base():
    """Debug base template"""
    return render_template("base.html")


@app.route("/")
def index():
    """Index/homepage"""
    return render_template("index.html", articles=articles)


@app.route("/battle/<int:id>")
def battle(id):
    for article in articles:
        if article["is_battle"] and article["id"] == id:  # is_battle just temp
            return render_template("battle.html", article=article)

    return redirect("/e404")


@app.route("/war/<int:war_num>")
def war(war_num):
    for article in articles:
        if not article["is_battle"] and article["war_num"] == war_num:
            return render_template("war.html", article=article)

    return redirect("/e404")


@app.route("/404")
def e404():
    return render_template("base.html")


app.run(debug=True, host="0.0.0.0", port=8080)
