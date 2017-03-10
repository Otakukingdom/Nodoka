//
// Created by mistlight on 3/9/2017.
//

#include "DatabaseModel.h"

DatabaseModel::DatabaseModel(std::shared_ptr<Core::DatabaseInstance> dbInstance) {
    this->dbInstance = dbInstance;
}

void DatabaseModel::writeObject(QString key, QJsonObject value) {
    auto env = this->dbInstance->getDbEnv();

    auto wtxn = lmdb::txn::begin(env);
    auto dbi = lmdb::dbi::open(wtxn, nullptr);
    dbi.put(wtxn, key, value);
    wtxn.commit();
}

void DatabaseModel::writeArray(QString key, QJsonArray value) {
    auto env = this->dbInstance->getDbEnv();

    auto wtxn = lmdb::txn::begin(env);
    auto dbi = lmdb::dbi::open(wtxn, nullptr);
    dbi.put(wtxn, key, value);
    wtxn.commit();
}

