from flask import Flask, jsonify
from flask_sqlalchemy import SQLAlchemy
from geoalchemy2 import Geometry
import os
import json

app = Flask(__name__)

# Database configuration
app.config['SQLALCHEMY_DATABASE_URI'] = os.environ.get("DATABASE_URL",
                                                       "postgresql+psycopg2://postgres:password@localhost:5432/postgres")
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False

db = SQLAlchemy(app)


class Meter(db.Model):
    __tablename__ = 'meter'

    id = db.Column(db.Integer, primary_key=True)
    position = db.Column(Geometry('POINT', srid=4326))  # Using GeoAlchemy2 for spatial data
    feeder_id = db.Column(db.Integer, db.ForeignKey('feeder.id'))

    def to_dict(self):
        # Convert the position to GeoJSON format
        return {
            'id': self.id,
            'position': json.loads(db.session.scalar(self.position.ST_AsGeoJSON())) if self.position else None,
            'feeder_id': self.feeder_id
        }


@app.route('/meters', methods=['GET'])
def get_meters():
    meters = Meter.query.all()
    return jsonify([meter.to_dict() for meter in meters])


if __name__ == '__main__':
    app.run(debug=True)
