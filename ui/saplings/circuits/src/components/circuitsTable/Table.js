/**
 * Copyright 2018-2021 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import React from 'react';
import PropTypes from 'prop-types';
import { Circuit } from '../../data/circuits';
import TableRow from './TableRow';
import TableHeader from './TableHeader';

import './CircuitsTable.scss';

const CircuitsTable = ({ circuits, dispatch }) => {
  let rows = (
    <tr key="empty">
      <td colSpan="6" className="no-circuits-msg">
        No circuits found
      </td>
    </tr>
  );

  if (circuits.length > 0) {
    rows = circuits.map(item => {
      return <TableRow key={item.id} circuit={item} />;
    });
  }

  return (
    <div className="table-container">
      <table className="circuits-table">
        <TableHeader dispatch={dispatch} circuits={circuits} />
        <tbody>{rows}</tbody>
      </table>
    </div>
  );
};

CircuitsTable.propTypes = {
  circuits: PropTypes.arrayOf(PropTypes.instanceOf(Circuit)).isRequired,
  dispatch: PropTypes.func.isRequired
};

export default CircuitsTable;
