//
// Created by mistlight on 3/9/2017.
//

#include "DatabaseModel.h"
#include <QDebug>
#include <QJsonDocument>

using namespace Database;

DatabaseModel::DatabaseModel(std::shared_ptr<Core::DatabaseInstance> dbInstance) {
    this->dbInstance = dbInstance;
}

void DatabaseModel::writeObject(QString key, QJsonObject value) {
    auto env = this->dbInstance->getDbEnv();

    QJsonDocument jsonDocument(value);

    auto wtxn = lmdb::txn::begin(env);
    auto dbi = lmdb::dbi::open(wtxn, nullptr);
    qDebug() << "key saved to as " << key.toUtf8().data();
    qDebug() << "value saved to as " << jsonDocument.toJson().data();
    dbi.put(wtxn, key.toUtf8().data(), jsonDocument.toJson().data());
    wtxn.commit();
}

void DatabaseModel::writeArray(QString key, QJsonArray value) {
    auto env = this->dbInstance->getDbEnv();

    QJsonDocument jsonDocument(value);

    auto wtxn = lmdb::txn::begin(env);
    auto dbi = lmdb::dbi::open(wtxn, nullptr);
    dbi.put(wtxn, key.toStdString(), jsonDocument.toJson().data());
    wtxn.commit();
}

QJsonObject DatabaseModel::getObject(QString key) {
    auto env = this->dbInstance->getDbEnv();

    auto rtxn = lmdb::txn::begin(env, nullptr, MDB_RDONLY);
    auto dbi = lmdb::dbi::open(rtxn, nullptr);

    auto cursor = lmdb::cursor::open(rtxn, dbi);
    std::string valueStr;
    dbi.get(rtxn, key.toStdString(), valueStr);

    qDebug() << "value is " << valueStr.c_str();


    return QJsonObject();
}
